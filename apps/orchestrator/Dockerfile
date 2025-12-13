# =================================================================
# APARATO: ORCHESTRATOR HYPER-BUILDER (V4.2)
# ESTRATEGIA: CARGO CHEF LOCKED + MULTI-STAGE OPTIMIZATION
# ESTADO: FIX APLICADO (Dependency Hell Mitigation)
# =================================================================

# --- ETAPA 1: PLANIFICADOR (COMPUTE RECIPE) ---
FROM rust:1.83-slim-bookworm as planner
WORKDIR /app

# 1.1. Instalación de dependencias del sistema
RUN apt-get update && apt-get install -y pkg-config libssl-dev

# 1.2. Instalación de Herramientas (BLINDADA)
# EL FIX: Usamos --locked para evitar que dependencias "bleeding edge" (como ignore v0.4.25)
# rompan el build requiriendo versiones de Rust inestables (2024 edition).
RUN cargo install cargo-chef --locked

COPY . .
# Analiza el workspace y crea una "receta" json de dependencias
RUN cargo chef prepare --recipe-path recipe.json

# --- ETAPA 2: CACHÉ BUILDER (COOK DEPENDENCIES) ---
FROM rust:1.83-slim-bookworm as cacher
WORKDIR /app

# Reinstalamos cargo-chef (necesario en esta etapa nueva) con el mismo blindaje
RUN apt-get update && apt-get install -y pkg-config libssl-dev
RUN cargo install cargo-chef --locked

COPY --from=planner /app/recipe.json recipe.json
# Compila TODAS las dependencias (libs externas) y las guarda en caché
RUN cargo chef cook --release --recipe-path recipe.json

# --- ETAPA 3: COMPILADOR FINAL (SOURCE CODE) ---
FROM rust:1.83-slim-bookworm as builder
WORKDIR /app
COPY . .
# Recuperamos la carpeta 'target' pre-compilada de la etapa anterior
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo

# Compilamos SOLO el código fuente de nuestra app (Ultra rápido)
RUN cargo build --release --bin orchestrator

# --- ETAPA 4: RUNTIME (PRODUCTION) ---
FROM debian:bookworm-slim as runtime
WORKDIR /app

# Dependencias mínimas del sistema operativo (SSL es obligatorio)
RUN apt-get update && \
    apt-get install -y ca-certificates libssl3 curl && \
    rm -rf /var/lib/apt/lists/*

# Lógica de Hidratación del Filtro (Gestión de Artefactos Externos)
ARG FILTER_URL
RUN if [ -n "$FILTER_URL" ]; then \
      echo "⬇️ Descargando Filtro UTXO..."; \
      curl -L -f --retry 3 -o utxo_filter.bin "$FILTER_URL"; \
    else \
      echo "⚠️ FILTER_URL no definida. Iniciando con filtro vacío."; \
      touch utxo_filter.bin; \
    fi

# Copiamos solo el binario final
COPY --from=builder /app/target/release/orchestrator /usr/local/bin/orchestrator

# Configuración de entorno segura
ENV RUST_LOG=info
ENV PORT=3000

EXPOSE 3000

# Healthcheck de bajo coste (sin auth)
HEALTHCHECK --interval=30s --timeout=5s \
  CMD curl -f http://localhost:3000/health || exit 1

CMD ["orchestrator"]
