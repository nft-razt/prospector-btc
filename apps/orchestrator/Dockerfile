# =================================================================
# APARATO: ORCHESTRATOR AUTÓNOMO
# CARACTERÍSTICA: SELF-PROVISIONING (Instala sus propias herramientas)
# =================================================================

# --- ETAPA 1: PREPARACIÓN DE HERRAMIENTAS ---
FROM rust:1.83-slim-bookworm as tools
WORKDIR /app
# 🔥 AQUÍ SE INSTALA AUTOMÁTICAMENTE EN EL SERVIDOR
RUN apt-get update && apt-get install -y pkg-config libssl-dev
RUN cargo install cargo-chef

# --- ETAPA 2: PLANIFICACIÓN (Receta de Dependencias) ---
FROM rust:1.83-slim-bookworm as planner
WORKDIR /app
# Copiamos el binario de cargo-chef desde la etapa 1
COPY --from=tools /usr/local/cargo/bin/cargo-chef /usr/local/cargo/bin/cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# --- ETAPA 3: CACHÉ BUILDER (Cocina de Dependencias) ---
FROM rust:1.83-slim-bookworm as cacher
WORKDIR /app
COPY --from=tools /usr/local/cargo/bin/cargo-chef /usr/local/cargo/bin/cargo-chef
COPY --from=planner /app/recipe.json recipe.json
# Esto tarda la primera vez, pero se cachea en Render para siempre
RUN cargo chef cook --release --recipe-path recipe.json

# --- ETAPA 4: COMPILACIÓN DEL CÓDIGO FUENTE ---
FROM rust:1.83-slim-bookworm as builder
WORKDIR /app
COPY . .
# Recuperamos dependencias cacheadas
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --release --bin orchestrator

# --- ETAPA 5: RUNTIME (Producción Ligera) ---
FROM debian:bookworm-slim as runtime
WORKDIR /app
RUN apt-get update && apt-get install -y ca-certificates libssl3 curl && rm -rf /var/lib/apt/lists/*

# Descarga del Filtro UTXO (Gestión de Artefacto Externo)
ARG FILTER_URL
RUN if [ -n "$FILTER_URL" ]; then \
      curl -L -f --retry 3 -o utxo_filter.bin "$FILTER_URL"; \
    else \
      echo "⚠️ FILTER_URL vacío. Iniciando sin filtro."; \
      touch utxo_filter.bin; \
    fi

COPY --from=builder /app/target/release/orchestrator /usr/local/bin/orchestrator

ENV RUST_LOG=info
ENV PORT=3000
EXPOSE 3000

HEALTHCHECK --interval=30s --timeout=5s CMD curl -f http://localhost:3000/health || exit 1
CMD ["orchestrator"]
