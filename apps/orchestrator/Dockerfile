# apps/orchestrator/Dockerfile
# =================================================================
# APARATO: ORCHESTRATOR HYPER-BUILDER (V5.0 - PATH FIX)
# ESTADO: REPARADO (CONFLICTO .cargo/config.toml ELIMINADO)
# =================================================================

# --- ETAPA 1: PLANIFICADOR ---
FROM rust:1.83-slim-bookworm as planner
WORKDIR /app
# Definimos variable de entorno global
ENV CARGO_TARGET_DIR=/app/target

RUN apt-get update && apt-get install -y pkg-config libssl-dev
RUN cargo install cargo-chef --locked

COPY . .
# 🔥 FIX CRÍTICO: Eliminamos la config local que apunta a dist/target
RUN rm -rf .cargo

RUN cargo chef prepare --recipe-path recipe.json

# --- ETAPA 2: CACHÉ BUILDER ---
FROM rust:1.83-slim-bookworm as cacher
WORKDIR /app
ENV CARGO_TARGET_DIR=/app/target

RUN apt-get update && apt-get install -y pkg-config libssl-dev
RUN cargo install cargo-chef --locked

COPY --from=planner /app/recipe.json recipe.json
# Ahora cargo obedecerá exclusivamente a CARGO_TARGET_DIR
RUN cargo chef cook --release --recipe-path recipe.json

# --- ETAPA 3: COMPILADOR FINAL ---
FROM rust:1.83-slim-bookworm as builder
WORKDIR /app
ENV CARGO_TARGET_DIR=/app/target

COPY . .
# 🔥 FIX CRÍTICO: Eliminamos la config local otra vez (porque copiamos . .)
RUN rm -rf .cargo

# Recuperamos la caché limpia
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
# Script de descarga tolerante a fallos (no rompe el build si falla, el código Rust lo maneja)
RUN if [ -n "$FILTER_URL" ]; then \
      echo "⬇️ [BUILD] Descargando Filtro UTXO..."; \
      curl -L -f --retry 3 --retry-delay 5 -o utxo_filter.bin "$FILTER_URL" || echo "⚠️ Descarga fallida, sistema iniciará en MANTENIMIENTO"; \
    else \
      echo "⚠️ [BUILD] FILTER_URL no definida. Usando modo MANTENIMIENTO."; \
    fi

COPY --from=builder /app/target/release/orchestrator /usr/local/bin/orchestrator

ENV RUST_LOG=info
ENV PORT=3000

EXPOSE 3000

# Healthcheck apuntando al endpoint público
HEALTHCHECK --interval=30s --timeout=5s --start-period=5s --retries=3 \
  CMD curl -f http://localhost:3000/health || exit 1

CMD ["orchestrator"]
