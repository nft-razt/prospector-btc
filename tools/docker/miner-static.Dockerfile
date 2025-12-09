
FROM rust:1.75-slim-bookworm as builder

# Instalar herramientas para compilación cruzada estática (MUSL)
RUN apt-get update && apt-get install -y musl-tools pkg-config libssl-dev

# Configurar target
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app
COPY . .

# Compilar solo el minero, estáticamente
# --release: Optimización máxima
# --target: Linux genérico (funciona en Colab/Alpine/Ubuntu)
RUN cargo build --release --target x86_64-unknown-linux-musl -p prospector-miner

# Extraer binario
# Al terminar, puedes copiarlo con:
# docker create --name extract prospector-miner-builder
# docker cp extract:/app/target/x86_64-unknown-linux-musl/release/miner-worker ./miner-musl
