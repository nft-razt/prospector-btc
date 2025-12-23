# =================================================================
# APARATO: PROVISIONER CLOUD WORKER
# VERSIÓN DE NODO OBJETIVO: 20.19.0
# =================================================================

FROM mcr.microsoft.com/playwright:v1.40.0-jammy

WORKDIR /app

# Auditoría de versión pre-instalación
RUN node --version

# 1. Capa de Dependencias (Cache optimizado)
COPY package.json pnpm-lock.yaml pnpm-workspace.yaml ./
COPY tools/provisioner/package.json ./tools/provisioner/

RUN npm install -g pnpm

# Instalamos dependencias del workspace
RUN pnpm install --frozen-lockfile --filter @prospector/provisioner...

# 2. Capa de Código
COPY . .

# 3. Configuración de Entorno Runtime
ENV HEADLESS=true
ENV CI=true
ENV PLAYWRIGHT_SKIP_BROWSER_DOWNLOAD=1

# 4. Ejecución
WORKDIR /app/tools/provisioner
CMD ["npx", "ts-node", "src/main.ts"]
