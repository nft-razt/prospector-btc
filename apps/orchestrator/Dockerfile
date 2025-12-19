# =================================================================
# APARATO: ORCHESTRATOR HYPER-BUILDER (V18.0)
# CLASIFICACIÓN: OPS INFRASTRUCTURE
# OBJETIVO: Generar un entorno de ejecución Zero-Trust y optimizado.
# =================================================================

# --- ETAPA 1: PLANNER (Caché de dependencias) ---
FROM rust:1.83-slim-bookworm as planner
WORKDIR /app
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# --- ETAPA 2: CACHER ---
FROM rust:1.83-slim-bookworm as cacher
WORKDIR /app
RUN apt-get update && apt-get install -y pkg-config libssl-dev
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# --- ETAPA 3: BUILDER (Compilación binaria múltiple) ---
FROM rust:1.83-slim-bookworm as builder
WORKDIR /app
RUN apt-get update && apt-get install -y pkg-config libssl-dev
COPY . .
# Copiamos la caché previa
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo

# Compilamos el orquestador y el migrador de forma atómica
RUN cargo build --release --bin orchestrator
RUN cargo build --release --bin migrator

# --- ETAPA 4: RUNTIME (Entorno de producción blindado) ---
FROM debian:bookworm-slim as runtime
WORKDIR /app

# Instalamos utilidades mínimas de diagnóstico para el protocolo verboso
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Inyección de binarios y scripts
COPY --from=builder /app/target/release/orchestrator .
COPY --from=builder /app/target/release/migrator ./prospector-migrator
COPY scripts/entrypoint.sh ./entrypoint.sh
RUN chmod +x ./entrypoint.sh

# Artefactos persistentes (Filtro UTXO)
# Se asume que el filtro se descarga en tiempo de build o se inyecta por volumen
ARG FILTER_URL
RUN if [ -n "$FILTER_URL" ]; then \
    echo "⬇️ Downloading Tactical Filter from $FILTER_URL..."; \
    curl -L -o utxo_filter.bin "$FILTER_URL"; \
    fi

# Configuración de red
ENV RUST_LOG=info
ENV PORT=3000
EXPOSE 3000

# Punto de entrada orquestado
ENTRYPOINT ["./entrypoint.sh"]
