📜 PROTOCOLO DE IGNICIÓN: DESPLIEGUE SOBERANO EN VERCEL (V10.8)

**FASE 1: AUDITORÍA DE INTEGRIDAD LOCAL (PRE-PUSH)**
[ ] pnpm install
[ ] pnpm audit:types:check
[ ] pnpm audit:logic:linter
[ ] cargo test -p prospector-core-math
[ ]pnpm i18n:generate

Acceder a Settings > Environment Variables en Vercel e inyectar los 6 estratos de secretos.

Estrato 1: Conectividad Táctica (Uplink)
**NEXT_PUBLIC_API_URL**: https://prospector-orchestrator.onrender.com/api/v1

**NEXT_PUBLIC_API_TOKEN**: Mismo valor que WORKER_AUTH_TOKEN.

**NEXT_PUBLIC_SUPABASE_URL**:
**NEXT_PUBLIC_SUPABASE_ANON_KEY**
**SUPABASE_SERVICE_ROLE_KEY**

**GITHUB_PAT**
**GITHUB_OWNER**
**GITHUB_REPO**

**AUTH_SECRET**
**NEXT_PUBLIC_ADMIN_PASSWORD**

**AUTH_GOOGLE_ID**
**AUTH_GOOGLE_SECRET**

**FILTER_BASE_URL**
**FILTER_SHARDS**


COMANDOS BUILD:
pnpm build:dashboard:production

Output Directory:
dist/apps/web-dashboard/.next

Root Directory:
Dejar en blanco (Raíz del monorepo).

Install Command:
pnpm install

FASE 5: CERTIFICACIÓN DE ENLACES (POST-DEPLOY)
Una vez que Vercel marque "Ready", ejecutar la suite de diagnóstico.
Validación de Túnel Neural:
Acceder a https://tu-app.vercel.app/api/github/runs

Acción: Verificar que devuelva un JSON (o 401 si no estás logueado), no un 404.

Prueba de Carga del Censo:
Ejecutar: ts-node tools/scripts/health-checks/check-strategic-link.ts
Razón: Certificar que el Dashboard puede leer las vistas de Postgres en Supabase.
Handshake de Motores Gemelos:

Ejecutar: ts-node tools/scripts/health-checks/check-sync-parity.ts
Acción: Confirmar que el drift sea reportado correctamente.
Smoke Test Visual:
Entrar al Dashboard -> Identity Vault.
Inyectar una cookie de prueba.
Verificar que el VaultCryptoEngine (L1) no lance excepciones de buffer.


🛠️ COMANDO ÚNICO DE PRE-LANZAMIENTO (SUIZO)
Para automatizar la revisión de seguridad antes del git push, corre este pipeline en tu terminal local:
code
Bash

# Limpieza, Generación de I18n, Linting, Type-checking y Test de Motores

pnpm install && pnpm i18n:generate && pnpm audit:logic:linter && pnpm audit:types:check && cargo test --workspace
