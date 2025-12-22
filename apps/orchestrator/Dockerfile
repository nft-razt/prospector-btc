# =================================================================
# APARATO: ORCHESTRATOR HYPER-BUILDER (V20.0 - PRODUCTION SECURED)
# CLASIFICACIÓN: OPS INFRASTRUCTURE
# RESPONSABILIDAD: CONSTRUCCIÓN DETERMINISTA Y RESILIENTE
#
# VISION HIPER-HOLÍSTICA:
# Utiliza una estrategia de construcción multi-etapa basada en
# cargo-chef para maximizar el aprovechamiento de la caché de capas
# en entornos CI/CD (Render/Koyeb). Garantiza un binario final
# optimizado y un entorno de ejecución ligero (Slim).
# =================================================================

# --- ETAPA 1: LIVENESS & CHEF BASE ---
# Implementa la base de compilación con cargo-chef pre-instalado.
FROM lukemathwalker/cargo-chef:latest-rust-1.83-bookworm AS chef
WORKDIR /app

# --- ETAPA 2: PLANNER (Análisis de dependencias) ---
# Genera la receta de dependencias para evitar recompilaciones innecesarias.
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# --- ETAPA 3: BUILDER (Compilación de Élite) ---
FROM chef AS builder
# Instalación de librerías de enlace para SSL y Protocolos de Red
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Fase de "Cooking": Compila solo las dependencias (Capa de caché pesada)
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Fase de "Assembly": Compilación de los binarios atómicos del proyecto
COPY . .
RUN cargo build --release --bin orchestrator
RUN cargo build --release --bin migrator

# --- ETAPA 4: RUNTIME (Entrato de Ejecución Blindado) ---
# Imagen final optimizada basada en Debian Bookworm Slim.
FROM debian:bookworm-slim AS runtime
WORKDIR /app

# Instalación de certificados CA y librerías runtime necesarias para HTTPS
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Inyección de artefactos desde la etapa de construcción
COPY --from=builder /app/target/release/orchestrator ./orchestrator
COPY --from=builder /app/target/release/migrator ./prospector-migrator
COPY scripts/entrypoint.sh ./entrypoint.sh

# Protocolo de permisos: Garantiza la ejecución de la ignición
RUN chmod +x ./entrypoint.sh

# Configuración de variables de entorno operativas
ENV RUST_LOG=info
ENV PORT=3000
EXPOSE 3000

# Punto de entrada soberano (Orquestación de migración y arranque)
ENTRYPOINT ["./entrypoint.sh"]
