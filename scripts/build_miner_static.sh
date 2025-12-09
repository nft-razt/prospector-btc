#!/bin/bash
# =================================================================
# SCRIPT: STATIC MINER BUILDER
# OBJETIVO: Generar binario x86_64-unknown-linux-musl sin dependencias
# =================================================================

set -e

echo "🏗️  INICIANDO COMPILACIÓN ESTÁTICA (MUSL)..."

# Verificar si Docker está corriendo
if ! docker info > /dev/null 2>&1; then
  echo "❌ Error: Docker no está corriendo."
  exit 1
fi

# Usamos la imagen oficial de Rust con soporte MUSL
# Montamos el código fuente actual y el caché de cargo para velocidad
docker run --rm -it \
  -v "$(pwd)":/home/rust/src \
  -v cargo-cache:/root/.cargo/registry \
  -w /home/rust/src \
  messense/rust-musl-cross:x86_64-musl \
  cargo build --release --bin miner-worker --target x86_64-unknown-linux-musl

echo "✅ Compilación completada."
echo "📦 Artefacto: target/x86_64-unknown-linux-musl/release/miner-worker"

# Verificar binario
echo "🔍 Verificando enlace estático:"
file target/x86_64-unknown-linux-musl/release/miner-worker
