# apps/orchestrator/Dockerfile
# =================================================================
# APARATO: BACKEND ORCHESTRATOR
# CLASIFICACIÓN: PRODUCTION / RENDER DEPLOY
# ARQUITECTURA: MULTI-STAGE BUILD (Rust + Debian Slim)
# =================================================================

# -----------------------------------------------------------------
# ETAPA 1: BUILDER (Compilación y Enlazado)
# -----------------------------------------------------------------
FROM rust:1.83-slim-bookworm as builder

# Directorio de trabajo en el contenedor de construcción
WORKDIR /usr/src/app

# 1.1. Instalación de dependencias del sistema necesarias para compilar
# - pkg-config & libssl-dev: Para compilar criptografía (OpenSSL/reqwest)
# - curl & git: Para descargar dependencias y el índice de crates
RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get install -y pkg-config libssl-dev curl git && \
    rm -rf /var/lib/apt/lists/*

# 1.2. Copia del Contexto del Monorepo
# Copiamos TODO (.) porque el build necesita acceso al workspace completo
# (libs/core, libs/domain, Cargo.toml raíz, etc.)
COPY . .

# 1.3. Diagnóstico de Entorno
RUN echo "🏗️  Iniciando compilación del Orquestador..." && \
    rustc --version

# 1.4. Compilación Release
# NOTA DE INGENIERÍA:
# - Se ha eliminado '--locked' para permitir la actualización del Cargo.lock
#   dentro del contenedor, corrigiendo la desincronización de dependencias.
# - '-p prospector-orchestrator': Compila solo este binario y sus dependencias.
RUN cargo build --release -p prospector-orchestrator

# -----------------------------------------------------------------
# ETAPA 2: RUNTIME (Entorno de Ejecución Ligero)
# -----------------------------------------------------------------
FROM debian:bookworm-slim

WORKDIR /app

# 2.1. Preparación del Sistema Operativo
# - ca-certificates: Vital para llamadas HTTPS a Turso y Google
# - libssl3: Librería dinámica requerida por el binario compilado
# - curl: Necesario para el Healthcheck y descarga del filtro
RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get install -y ca-certificates libssl3 curl && \
    rm -rf /var/lib/apt/lists/*

# 2.2. Estructura de Archivos Estáticos
# Creamos el directorio donde vivirá el filtro de Bloom
RUN mkdir -p public

# 2.3. Inyección del Filtro UTXO (Estrategia Externa)
# Si se provee FILTER_URL en tiempo de build, se descarga.
# Si no, se crea un archivo vacío para evitar crashes al arrancar.
ARG FILTER_URL=""
RUN if [ -n "$FILTER_URL" ]; then \
      echo "⬇️ Descargando filtro desde CDN..."; \
      curl -L -f -o public/utxo_filter.bin "$FILTER_URL"; \
    else \
      echo "⚠️  FILTER_URL no provista. Generando placeholder vacío."; \
      touch public/utxo_filter.bin; \
    fi

# 2.4. Instalación del Binario (CORRECCIÓN CRÍTICA DE RUTA)
# Nx y .cargo/config.toml redirigen la salida a 'dist/target'.
# Copiamos desde esa ubicación específica.
COPY --from=builder /usr/src/app/dist/target/release/orchestrator /usr/local/bin/orchestrator

# 2.5. Configuración de Entorno
ENV RUST_LOG=info
ENV PORT=3000

# Exposición de puerto (Documentación para Docker)
EXPOSE 3000

# 2.6. Monitor de Salud (Healthcheck)
# Render/Docker usarán esto para saber si el contenedor está listo.
HEALTHCHECK --interval=30s --timeout=5s --start-period=5s \
  CMD curl -f http://localhost:3000/api/v1/status || exit 1

# 2.7. Comando de Inicio
CMD ["orchestrator"]
