#!/bin/bash
# =================================================================
# APARATO: STATIC MINER BUILDER (ELITE EDITION)
# OBJETIVO: Generar binario x86_64-unknown-linux-musl sin dependencias
# OPTIMIZACIÓN: Permisos de usuario, Clean Build, Linkado Estático
# =================================================================

set -e

# Definición de rutas y constantes
TARGET_DIR="dist/target"
OUTPUT_BIN="$TARGET_DIR/x86_64-unknown-linux-musl/release/miner-worker"
SOURCE_BIN="apps/miner-worker"

echo "🏗️  INICIANDO PROTOCOLO DE COMPILACIÓN ESTÁTICA (MUSL)..."

# 1. Verificación de Docker
if ! docker info > /dev/null 2>&1; then
  echo "❌ Error: Docker no está corriendo o no tienes permisos."
  exit 1
fi

# 2. Limpieza de artefactos previos (Clean Slate)
if [ -f "$OUTPUT_BIN" ]; then
    echo "🧹 Eliminando binario anterior..."
    rm -f "$OUTPUT_BIN"
fi

# 3. Compilación en Contenedor Efímero
# Mapeamos el usuario actual para evitar archivos propiedad de 'root' en ./target
USER_ID=$(id -u)
GROUP_ID=$(id -g)

echo "🔄 Lanzando contenedor de compilación (Cross-Compilation)..."
echo "   - User ID: $USER_ID"
echo "   - Target: x86_64-unknown-linux-musl"

docker run --rm -it \
  -u "$USER_ID:$GROUP_ID" \
  -v "$(pwd)":/home/rust/src \
  -v cargo-cache:/home/rust/.cargo/registry \
  -w /home/rust/src \
  -e RUSTFLAGS='-C target-feature=+crt-static' \
  messense/rust-musl-cross:x86_64-musl \
  cargo build --release --bin miner-worker --target x86_64-unknown-linux-musl

# 4. Verificación de Integridad
if [ -f "$OUTPUT_BIN" ]; then
    echo "✅ COMPILACIÓN EXITOSA."
    echo "📦 Artefacto generado en: $OUTPUT_BIN"

    # Análisis forense del binario
    echo "🔍 Análisis de Enlace:"
    file "$OUTPUT_BIN"

    SIZE=$(du -h "$OUTPUT_BIN" | cut -f1)
    echo "⚖️  Tamaño del Binario: $SIZE"
else
    echo "❌ ERROR: El binario no fue generado."
    exit 1
fi
