# apps/orchestrator/Dockerfile
# =================================================================
# APARATO: ORCHESTRATOR HYPER-BUILDER (V4.6 - PATH FIX)
# ESTRATEGIA: STANDARDIZED TARGET DIR
# CORRECCIÓN: Forzamos CARGO_TARGET_DIR para evitar conflicto con .cargo/config.toml
# =================================================================

# --- ETAPA 1: PLANIFICADOR ---
FROM rust:1.83-slim-bookworm as planner
WORKDIR /app
# 1. Definimos el directorio de salida explícito para evitar conflictos con config local
ENV CARGO_TARGET_DIR=/app/target

RUN apt-get update && apt-get install -y pkg-config libssl-dev
RUN cargo install cargo-chef --locked

COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# --- ETAPA 2: CACHÉ BUILDER ---
FROM rust:1.83-slim-bookworm as cacher
WORKDIR /app
# 2. Mantenemos la misma variable de entorno
ENV CARGO_TARGET_DIR=/app/target

RUN apt-get update && apt-get install -y pkg-config libssl-dev
RUN cargo install cargo-chef --locked

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# --- ETAPA 3: COMPILADOR FINAL ---
FROM rust:1.83-slim-bookworm as builder
WORKDIR /app
# 3. Y aquí también
ENV CARGO_TARGET_DIR=/app/target

COPY . .
# Recuperamos la carpeta target normalizada
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo

RUN cargo build --release --bin orchestrator

# --- ETAPA 4: RUNTIME ---
FROM debian:bookworm-slim as runtime
WORKDIR /app

RUN apt-get update && \
    apt-get install -y ca-certificates libssl3 curl && \
    rm -rf /var/lib/apt/lists/*

ARG FILTER_URL
RUN if [ -n "$FILTER_URL" ]; then \
      echo "⬇️ [BUILD] Descargando Filtro UTXO..."; \
      curl -L -f --retry 3 --retry-delay 5 -o utxo_filter.bin "$FILTER_URL"; \
      FILE_SIZE=$(stat -c%s utxo_filter.bin); \
      if [ "$FILE_SIZE" -lt 1048576 ]; then \
          echo "❌ [BUILD] ERROR: Filtro corrupto o muy pequeño."; \
          exit 1; \
      else \
          echo "✅ [BUILD] Filtro verificado: $FILE_SIZE bytes."; \
      fi \
    else \
      echo "⚠️ [BUILD] FILTER_URL no definida. Usando dummy."; \
      touch utxo_filter.bin; \
    fi

# La ruta de origen ahora es estándar
COPY --from=builder /app/target/release/orchestrator /usr/local/bin/orchestrator

ENV RUST_LOG=info
ENV PORT=3000

EXPOSE 3000

HEALTHCHECK --interval=30s --timeout=5s --start-period=5s --retries=3 \
  CMD curl -f http://localhost:3000/health || exit 1

CMD ["orchestrator"]
