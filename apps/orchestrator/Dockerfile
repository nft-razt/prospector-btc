# apps/orchestrator/Dockerfile
# =================================================================
# APARATO: BACKEND ORCHESTRATOR
# CLASIFICACIÓN: PRODUCTION / RENDER DEPLOY
# ARQUITECTURA: MULTI-STAGE BUILD (Rust + Debian Slim)
# ESTADO: HARDENED (FAIL-FAST FILTER DOWNLOAD)
# =================================================================

# -----------------------------------------------------------------
# ETAPA 1: BUILDER
# -----------------------------------------------------------------
FROM rust:1.83-slim-bookworm as builder

WORKDIR /usr/src/app

# Instalar dependencias de compilación
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev curl git && \
    rm -rf /var/lib/apt/lists/*

COPY . .

# Compilación Release (Optimizada)
RUN echo "🏗️  Iniciando compilación del Orquestador..." && \
    cargo build --release -p prospector-orchestrator

# -----------------------------------------------------------------
# ETAPA 2: RUNTIME
# -----------------------------------------------------------------
FROM debian:bookworm-slim

WORKDIR /app

# Instalar dependencias de runtime (SSL + CA Certs vitales)
RUN apt-get update && \
    apt-get install -y ca-certificates libssl3 curl && \
    rm -rf /var/lib/apt/lists/*

# Crear estructura pública
RUN mkdir -p public

# INYECCIÓN DEL FILTRO UTXO (HARDENED)
# Regla: Si FILTER_URL se provee, DEBE descargarse correctamente.
# Si falla, el build falla. No permitimos "placeholders" en producción.
ARG FILTER_URL=""
RUN if [ -n "$FILTER_URL" ]; then \
      echo "⬇️ Descargando filtro desde CDN con reintentos..."; \
      curl -L -f --retry 3 --retry-delay 5 -o public/utxo_filter.bin "$FILTER_URL" || exit 1; \
      ls -lh public/utxo_filter.bin; \
    else \
      echo "⚠️  ADVERTENCIA: FILTER_URL no provista. El sistema iniciará en modo 'Ciego' (Solo desarrollo)."; \
      touch public/utxo_filter.bin; \
    fi

# Copiar binario desde el builder
COPY --from=builder /usr/src/app/dist/target/release/orchestrator /usr/local/bin/orchestrator

# Configuración de Entorno
ENV RUST_LOG=info,prospector_orchestrator=info
ENV PORT=3000

EXPOSE 3000

# LIVENESS PROBE (Sin Auth, ligero)
HEALTHCHECK --interval=30s --timeout=5s --start-period=5s \
  CMD curl -f http://localhost:3000/health || exit 1

CMD ["orchestrator"]
