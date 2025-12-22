# =================================================================
# APARATO: ORCHESTRATOR HYPER-BUILDER (V20.0 - SOBERANO)
# CLASIFICACIÓN: OPS INFRASTRUCTURE
# RESPONSABILIDAD: CONSTRUCCIÓN DETERMINISTA Y RESILIENTE
#
# VISION HIPER-HOLÍSTICA:
# Utiliza el patrón 'Cargo-Chef' para separar la compilación de
# dependencias del código fuente, reduciendo el tiempo de build
# en Render de 20 minutos a menos de 3 minutos tras la primera ignición.
# =================================================================

# --- ETAPA 1: PLANNER (Analista de Manifiesto) ---
FROM lukemathwalker/cargo-chef:latest-rust-1.83-bookworm AS planner
WORKDIR /app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# --- ETAPA 2: BUILDER (Constructor de Élite) ---
FROM lukemathwalker/cargo-chef:latest-rust-1.83-bookworm AS builder
WORKDIR /app

# Instalación de dependencias de enlazado y certificados
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=planner /app/recipe.json recipe.json
# Construcción del esqueleto (Caché de dependencias masiva)
RUN cargo chef cook --release --recipe-path recipe.json

# Inyección del código fuente sincronizado (Snapshot V10.8)
COPY . .
RUN cargo build --release --bin orchestrator

# --- ETAPA 3: RUNTIME (Entrato de Ejecución Blindado) ---
FROM debian:bookworm-slim AS runtime
WORKDIR /app

# Instalación de librerías compartidas mínimas para SSL
RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Copia de artefactos binarios y scripts de mando
COPY --from=builder /app/target/release/orchestrator ./prospector-orchestrator
COPY scripts/entrypoint.sh ./entrypoint.sh

# Garantizar soberanía de ejecución sobre los scripts
RUN chmod +x ./entrypoint.sh

# Configuración de variables operativas de Render
ENV RUST_LOG=info
ENV PORT=3000
EXPOSE 3000

# Punto de ignición orquestado
ENTRYPOINT ["./entrypoint.sh"]
