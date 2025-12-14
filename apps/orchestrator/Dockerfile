# apps/orchestrator/Dockerfile
# =================================================================
# APARATO: ORCHESTRATOR HYPER-BUILDER (V4.5 - FORTIFIED)
# ESTRATEGIA: CARGO CHEF LOCKED + INTEGRITY CHECKS
# CAMBIOS: Validación de tamaño de filtro y manejo estricto de errores
# =================================================================

# --- ETAPA 1: PLANIFICADOR (COMPUTE RECIPE) ---
FROM rust:1.83-slim-bookworm as planner
WORKDIR /app

# 1.1. Instalación de dependencias del sistema
RUN apt-get update && apt-get install -y pkg-config libssl-dev

# 1.2. Instalación de Herramientas (BLINDADA)
RUN cargo install cargo-chef --locked

COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# --- ETAPA 2: CACHÉ BUILDER (COOK DEPENDENCIES) ---
FROM rust:1.83-slim-bookworm as cacher
WORKDIR /app

RUN apt-get update && apt-get install -y pkg-config libssl-dev
RUN cargo install cargo-chef --locked

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# --- ETAPA 3: COMPILADOR FINAL (SOURCE CODE) ---
FROM rust:1.83-slim-bookworm as builder
WORKDIR /app
COPY . .
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo

RUN cargo build --release --bin orchestrator

# --- ETAPA 4: RUNTIME (PRODUCTION) ---
FROM debian:bookworm-slim as runtime
WORKDIR /app

# Dependencias mínimas (curl para healthcheck y descarga)
RUN apt-get update && \
    apt-get install -y ca-certificates libssl3 curl && \
    rm -rf /var/lib/apt/lists/*

# Lógica de Hidratación del Filtro con VALIDACIÓN DE INTEGRIDAD
ARG FILTER_URL
RUN if [ -n "$FILTER_URL" ]; then \
      echo "⬇️ [BUILD] Descargando Filtro UTXO desde fuente externa..."; \
      curl -L -f --retry 3 --retry-delay 5 -o utxo_filter.bin "$FILTER_URL"; \
      \
      # Validación de Tamaño (Evitar archivos corruptos/vacíos) \
      FILE_SIZE=$(stat -c%s utxo_filter.bin); \
      if [ "$FILE_SIZE" -lt 1048576 ]; then \
          echo "❌ [BUILD] ERROR CRÍTICO: El filtro descargado es sospechosamente pequeño ($FILE_SIZE bytes)."; \
          echo "   Verifique que la URL sea un enlace directo al binario (RAW) y no una página HTML."; \
          exit 1; \
      else \
          echo "✅ [BUILD] Filtro verificado: $FILE_SIZE bytes."; \
      fi \
    else \
      echo "⚠️ [BUILD] FILTER_URL no definida. Iniciando con filtro dummy (SOLO PARA DEV)."; \
      touch utxo_filter.bin; \
    fi

# Copiamos binario
COPY --from=builder /app/target/release/orchestrator /usr/local/bin/orchestrator

# Configuración de entorno segura
ENV RUST_LOG=info
ENV PORT=3000

EXPOSE 3000

# Healthcheck PÚBLICO (Sin Auth) y resiliente
HEALTHCHECK --interval=30s --timeout=5s --start-period=5s --retries=3 \
  CMD curl -f http://localhost:3000/health || exit 1

CMD ["orchestrator"]
