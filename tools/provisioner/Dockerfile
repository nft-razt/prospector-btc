# =================================================================
# APARATO: PROVISIONER CLOUD WORKER
# BASE: Playwright Official (Incluye Browsers + Node 20)
# =================================================================

FROM mcr.microsoft.com/playwright:v1.40.0-jammy

WORKDIR /app

# 1. Capa de Dependencias (Cache optimizado)
# Copiamos solo los archivos de definición primero
COPY package.json pnpm-lock.yaml pnpm-workspace.yaml ./
COPY tools/provisioner/package.json ./tools/provisioner/

# Instalamos PNPM
RUN npm install -g pnpm

# Instalamos dependencias del workspace (filtrando solo lo necesario)
RUN pnpm install --frozen-lockfile --filter @prospector/provisioner...

# 2. Capa de Código
COPY . .

# 3. Configuración de Entorno Runtime
ENV HEADLESS=true
ENV CI=true
# Evita que Playwright intente descargar navegadores de nuevo (ya están en la imagen base)
ENV PLAYWRIGHT_SKIP_BROWSER_DOWNLOAD=1

# 4. Ejecución
WORKDIR /app/tools/provisioner
CMD ["npx", "ts-node", "src/main.ts"]
