# apps/web-dashboard/Dockerfile
# =================================================================
# APARATO: FRONTEND CONTAINER (NEXT.JS STANDALONE)
# VERSIÓN DE NODO: 20.19.0 (Sincronizado con Bitácora V17.0)
# =================================================================

# --- 1. BASE ---
FROM node:20.19.0-alpine AS base
RUN apk add --no-cache libc6-compat
WORKDIR /app

# --- 2. DEPS ---
FROM base AS deps
COPY package.json pnpm-lock.yaml pnpm-workspace.yaml ./
RUN npm install -g pnpm && pnpm i --frozen-lockfile

# --- 3. BUILDER ---
FROM base AS builder
WORKDIR /app
COPY --from=deps /app/node_modules ./node_modules
COPY . .

# Variables de Build para Next.js
ENV NEXT_TELEMETRY_DISABLED 1
ENV NODE_ENV production

# Construcción optimizada
RUN echo "🏗️  Construyendo Dashboard bajo Node 20.19.0..."
RUN npm install -g pnpm && npx nx build web-dashboard --prod --skip-nx-cache

# --- 4. RUNNER ---
FROM base AS runner
WORKDIR /app

ENV NODE_ENV production
ENV NEXT_TELEMETRY_DISABLED 1
ENV PORT 3000

# Usuario no-root (Seguridad)
RUN addgroup --system --gid 1001 nodejs
RUN adduser --system --uid 1001 nextjs

# Copiar artefactos standalone
COPY --from=builder --chown=nextjs:nodejs /app/dist/apps/web-dashboard/.next/standalone ./
COPY --from=builder --chown=nextjs:nodejs /app/dist/apps/web-dashboard/.next/static ./dist/apps/web-dashboard/.next/static
COPY --from=builder --chown=nextjs:nodejs /app/apps/web-dashboard/public ./apps/web-dashboard/public

USER nextjs

EXPOSE 3000

CMD ["node", "apps/web-dashboard/server.js"]
