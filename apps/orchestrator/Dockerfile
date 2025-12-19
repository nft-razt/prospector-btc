/**
 * =================================================================
 * APARATO: ORCHESTRATOR HYPER-BUILDER (V19.0 - STABLE EDITION)
 * CLASIFICACIÓN: OPS INFRASTRUCTURE
 * RESPONSABILIDAD: CONSTRUCCIÓN DETERMINISTA Y RESILIENTE
 *
 * MEJORAS DE ÉLITE:
 * - Pre-compiled Chef: Utilizamos la imagen oficial de lukemathwalker para
 *   evitar conflictos de edición (Edition 2024 mismatch).
 * - Multi-Stage Optimization: Reducción drástica del tamaño de imagen final.
 * - Absolute Path Resolution: Garantiza que los binarios se localicen en /app.
 * =================================================================
 */

# --- ETAPA 1: LIVENESS & CHEF BASE ---
# Usamos la imagen oficial que ya tiene cargo-chef instalado y optimizado
FROM lukemathwalker/cargo-chef:latest-rust-1.83-bookworm AS chef
WORKDIR /app

# --- ETAPA 2: PLANNER (Análisis de dependencias) ---
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# --- ETAPA 3: BUILDER (Compilación con caché de capas) ---
FROM chef AS builder
# Instalación de dependencias de sistema para enlace (SSL/PkgConfig)
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

COPY --from=planner /app/recipe.json recipe.json
# Construimos solo las dependencias para cachear este paso (Costo computacional pesado)
RUN cargo chef cook --release --recipe-path recipe.json

# Copiamos el código fuente y compilamos los binarios atómicos
COPY . .
RUN cargo build --release --bin orchestrator
RUN cargo build --release --bin migrator

# --- ETAPA 4: RUNTIME (Entorno de ejecución blindado) ---
FROM debian:bookworm-slim AS runtime
WORKDIR /app

# Instalación de utilidades críticas y certificados de confianza
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Inyección de binarios desde el builder
COPY --from=builder /app/target/release/orchestrator ./orchestrator
COPY --from=builder /app/target/release/migrator ./prospector-migrator
COPY scripts/entrypoint.sh ./entrypoint.sh

# Asegurar permisos de ejecución para el orquestador de ignición
RUN chmod +x ./entrypoint.sh

# Configuración de variables de entorno de producción
ENV RUST_LOG=info
ENV PORT=3000
EXPOSE 3000

# Punto de entrada orquestado (Protocolo de arranque verboso)
ENTRYPOINT ["./entrypoint.sh"]
