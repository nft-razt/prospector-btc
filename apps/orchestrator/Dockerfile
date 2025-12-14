# apps/orchestrator/Dockerfile
# =================================================================
# APARATO: ORCHESTRATOR HYPER-BUILDER (V5.1 - DEBUGGER EDITION)
# OBJETIVO: Trazabilidad total de la descarga del filtro
# =================================================================

# --- ETAPA 1: PLANIFICADOR ---
FROM rust:1.83-slim-bookworm as planner
WORKDIR /app
ENV CARGO_TARGET_DIR=/app/target
RUN apt-get update && apt-get install -y pkg-config libssl-dev
RUN cargo install cargo-chef --locked
COPY . .
RUN rm -rf .cargo
RUN cargo chef prepare --recipe-path recipe.json

# --- ETAPA 2: CACHÉ BUILDER ---
FROM rust:1.83-slim-bookworm as cacher
WORKDIR /app
ENV CARGO_TARGET_DIR=/app/target
RUN apt-get update && apt-get install -y pkg-config libssl-dev
RUN cargo install cargo-chef --locked
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# --- ETAPA 3: COMPILADOR FINAL ---
FROM rust:1.83-slim-bookworm as builder
WORKDIR /app
ENV CARGO_TARGET_DIR=/app/target
COPY . .
RUN rm -rf .cargo
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --release --bin orchestrator

# --- ETAPA 4: RUNTIME ---
FROM debian:bookworm-slim as runtime
WORKDIR /app

RUN apt-get update && \
    apt-get install -y ca-certificates libssl3 curl && \
    rm -rf /var/lib/apt/lists/*

# RECUPERAMOS LA VARIABLE DE ENTORNO
ARG FILTER_URL

# 🔍 BLOQUE DE DESCARGA CON DIAGNÓSTICO FORENSE
RUN echo "============== [ DEBUG INFO ] ==============" && \
    echo "URL Objetivo: '${FILTER_URL}'" && \
    if [ -z "$FILTER_URL" ]; then \
        echo "❌ ERROR: FILTER_URL está vacía. El build fallará."; \
        exit 1; \
    fi && \
    echo "⬇️ Iniciando descarga..." && \
    curl -v -L -f --retry 3 --retry-delay 5 -o utxo_filter.bin "$FILTER_URL" && \
    echo "✅ Descarga completada." && \
    ls -lh utxo_filter.bin && \
    echo "============================================"

COPY --from=builder /app/target/release/orchestrator /usr/local/bin/orchestrator

ENV RUST_LOG=info
ENV PORT=3000

EXPOSE 3000

HEALTHCHECK --interval=30s --timeout=5s --start-period=5s --retries=3 \
  CMD curl -f http://localhost:3000/health || exit 1

CMD ["orchestrator"]
