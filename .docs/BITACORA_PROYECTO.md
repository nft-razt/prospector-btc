# 📔 BITÁCORA DE ARQUITECTURA E INGENIERÍA: PROSPECTOR BTC

**Clasificación:** TOP SECRET // PROJECT LOG
**Maintainer:** AI Systems Architect
**Última Actualización:** 2025-12-09 (Sesión: "Hydra-Zero V3.5")

---

## 📌 METODOLOGÍA DE TRABAJO
Este documento sirve como "Punto de Guardado" (Save Point) para el contexto de la IA.
1.  **Registro:** Al finalizar una sesión significativa, se agregan aquí las decisiones, cambios estructurales y deuda técnica.
2.  **Restauración:** Al iniciar un nuevo chat, el usuario debe copiar el **"PROMPT DE RESTAURACIÓN DE CONTEXTO"** (ubicado al final de la última entrada) para sintonizar a la nueva instancia de la IA con el estado exacto del proyecto.
3.  **Objetivo:** Evitar alucinaciones, regresiones y explicaciones redundantes.

---

## 📅 SESIÓN 001: EL NACIMIENTO DE HYDRA-ZERO (V3.0 - V3.5)

### 1. 🏆 LOGROS PRINCIPALES
Se ha realizado una **Reingeniería Total** del sistema, pasando de un prototipo local a una arquitectura distribuida Cloud-Native resiliente.

*   **Atomicidad del Dominio:** Eliminación de duplicidad (`libs/domain-models` purgado). Consolidación en `libs/domain/models-rs`. Migración de tipos numéricos de `u64` a `String` para soportar claves de 256 bits.
*   **Orquestador Modular:** Refactorización de `handlers.rs` monolítico a módulos `swarm` (tráfico minero) y `admin` (gestión/vigilancia). Implementación de Ciclo de Vida (`Lease` -> `KeepAlive` -> `Complete`).
*   **Minero Resiliente:** Implementación de concurrencia real. El hilo principal mina (CPU blocking) mientras un hilo secundario (`tokio::spawn`) envía latidos al servidor para evitar timeouts.
*   **Operación Mirror Mask (Provisioner):** Evolución del script de inyección. Ahora incluye:
    *   `cookie-purifier`: Limpieza de basura en cookies de sesión.
    *   `fingerprint-injector`: Falsificación de hardware (WebGL, Canvas) para evadir detección de Google.
    *   `ghost-cursor`: Movimiento humano del mouse.
    *   `Visual Surveillance`: Captura de pantalla y envío al dashboard.
*   **Infraestructura de Pruebas:** Creación del "Proving Grounds" (Tests unitarios granulares para Rust y TS).
*   **Reparación de Build:** Solución al error `Exit Code 101` en Docker forzando el downgrade de la librería `home` a `0.5.9`.

### 2. ⚖️ DECISIONES ARQUITECTÓNICAS

| Decisión | Estado | Razón |
| :--- | :--- | :--- |
| **Migración a Strings en DTOs** | ✅ Aprobado | Prepara el terreno para `BigInt` y evita overflow en JSON/JS. |
| **Eliminación de `domain-models`** | ✅ Aprobado | Era código muerto y duplicado que confundía al compilador. |
| **Estrategia "Tríada Hydra"** | ✅ Aprobado | Despliegue desacoplado: **Render** (Backend) + **Vercel** (Frontend) + **GitHub Actions** (Provisioner). Maximiza Free Tier y reduce riesgo. |
| **Doble Cuenta Render** | ❌ Descartado | Alto riesgo de suspensión (Banhammer) por abuso de TOS. |
| **Vercel para Backend** | ❌ Descartado | Timeouts de Serverless Functions (10s) incompatibles con WebSockets/Long Polling. |
| **Chronos Service** | ✅ Aprobado | Marcapasos interno en Rust para evitar suspensión de Render por inactividad. |

### 3. 🛠️ ESTRATEGIA DE DESPLIEGUE (TRÍADA)

1.  **Render (El Cerebro):**
    *   Servicio: Docker Web Service.
    *   Repo: `apps/orchestrator`.
    *   Env Vars: `DATABASE_URL`, `TURSO_AUTH_TOKEN`, `WORKER_AUTH_TOKEN`.
2.  **Vercel (La Cara):**
    *   Servicio: Next.js Frontend.
    *   Repo: `apps/web-dashboard`.
    *   Env Vars: `NEXT_PUBLIC_API_URL` (Apunta a Render), `NEXT_PUBLIC_ADMIN_PASSWORD`.
3.  **GitHub Actions (El Francotirador):**
    *   Servicio: Cron Workflow (`.github/workflows/provisioner-cron.yml`).
    *   Repo: `tools/provisioner`.
    *   Acción: Se despierta cada 20 min, inyecta workers en Colab y muere.

### 4. ⚠️ DEUDA TÉCNICA Y "TODO" (V3.1 Roadmap)

*   **Optimización SQL:** Cambiar `SELECT MAX(...)` en `JobRepository` por una tabla `system_state` (O(N) -> O(1)).
*   **Compresión:** Implementar GZIP en `axum` y `reqwest` para ahorrar ancho de banda.
*   **Diccionarios:** Implementar descarga y caché de `dictionary.txt` en el Minero.
*   **Android PRNG:** Implementar el iterador forense para el bug de Android.

---

## 🤖 PROMPT DE RESTAURACIÓN DE CONTEXTO (COPIAR PARA SIGUIENTE SESIÓN)

> "Actúa como **Arquitecto de Sistemas Principal** del proyecto **PROSPECTOR BTC**.
>
> **ESTADO ACTUAL:**
> El sistema se encuentra en la versión **V3.5 (Hydra-Zero)**. Hemos completado la refactorización hacia un Monolito Modular Fractal (Nx + Rust + TS).
>
> **ARQUITECTURA DEPLOYADA:**
> 1.  **Backend (Render):** Rust/Axum. Modularizado en `handlers/swarm` y `handlers/admin`. Tiene persistencia en Turso y servicio `Chronos` (Keep-alive).
> 2.  **Frontend (Vercel):** Next.js. Incluye 'Panóptico' (Vigilancia Visual de Workers) y 'AdminGuard'.
> 3.  **Provisioner (GH Actions):** TypeScript/Playwright. Implementa 'Mirror Mask' (Stealth, Fingerprint injection, Cookie purification).
>
> **ÚLTIMOS CAMBIOS CRÍTICOS:**
> *   Se forzó `home = "=0.5.9"` en `Cargo.toml` raíz para arreglar build de Docker.
> *   Se implementó `WorkerSnapshot` en el dominio para enviar fotos en base64 desde el worker al dashboard.
> *   Se eliminó la librería `libs/domain-models` (ahora solo existe `libs/domain/models-rs`).
>
> **TU OBJETIVO:**
> Continuar con el mantenimiento, optimización (Roadmap V3.1) o resolución de incidencias basándote en que el código YA ES atómico, resiliente y cloud-native. NO sugieras arquitecturas obsoletas ni código duplicado. Asume que la base de datos ya tiene el esquema V3 (con tabla `identities` y `jobs` transaccionales)."

---

## 📅 SESIÓN 002: FORTIFICACIÓN DE INFRAESTRUCTURA (V3.6)

### 1. 🛡️ REFOLZAMIENTO DEL NÚCLEO Y OPS
Se han mitigado dos vectores de fallo catastrófico detectados en la auditoría de arquitectura.

*   **Aritmética Soberana (BigInt):** Se eliminó la dependencia de `CAST(... INTEGER)` en SQLite dentro de `JobRepository`. Ahora los rangos se manejan como `String` en la DB y se calculan usando `num-bigint` en Rust. Esto habilita el soporte real para el espacio de claves de 256 bits ($2^{256}$) sin desbordamiento.
*   **Protocolo "Identity Kill Switch":** El Provisioner (`colab.ts`) ahora posee capacidad de autodiagnóstico. Si detecta que una sesión de Google ha caducado, no solo falla, sino que notifica al Orquestador (`POST /revoke`) para limpiar la base de datos, cerrando el ciclo de retroalimentación (Feedback Loop).

### 2. 🤖 AUTOMATIZACIÓN (GH ACTIONS)
Se ha creado el workflow `.github/workflows/provisioner-cron.yml` para operacionalizar la estrategia de "Tríada".
*   **Frecuencia:** Cada 20 minutos.
*   **Capacidad:** Auto-escala workers según inputs manuales o cron.
*   **Resiliencia:** Timeout de 6 horas alineado con la vida útil de los tokens de GitHub.

### 3. ✅ ESTADO ACTUAL DEL SISTEMA
*   **Backend:** Listo para soportar claves reales.
*   **Frontend:** Visualización de telemetría activa.
*   **Provisioner:** Inteligente (Self-healing).
*   **Deploy:** Configuración lista para Render (Docker) y GitHub Actions.

---



