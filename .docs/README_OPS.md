# 🛠️ PROSPECTOR OPS MANUAL: DEPLOYMENT PROTOCOL

Este documento detalla la configuración operativa para desplegar el enjambre **Hydra-Zero**.

## 1. Configuración de Secretos (GitHub)

Para que el `Provisioner` funcione en GitHub Actions, debes configurar los siguientes **Repository Secrets** en tu repo (`Settings > Secrets and variables > Actions`).

| Secreto | Descripción | Ejemplo |
| :--- | :--- | :--- |
| `ORCHESTRATOR_URL` | URL pública de tu backend en Render. | `https://prospector-api.onrender.com` |
| `WORKER_AUTH_TOKEN` | Token maestro compartido. | `mi_secreto_super_seguro_v1` |
| `MINER_BINARY_URL` | Enlace directo al binario `miner-worker` compilado estáticamente. | `https://github.com/Usuario/Repo/releases/download/v1/miner-worker` |
| `GOOGLE_COOKIES_JSON` | Cookies de sesión de Google (Opcional pero recomendado). | `[{"domain": ".google.com", ...}]` |

> **⚠️ IMPORTANTE:** El `MINER_BINARY_URL` debe ser accesible públicamente (GitHub Release o R2/S3 público). Si usas GitHub Releases en un repo privado, necesitarás un token en la URL.

## 2. Compilación del Binario Estático (MUSL)

Antes de lanzar el enjambre, debes compilar el minero para que sea compatible con los contenedores Linux de Google Colab.

**Desde Windows (Powershell):**
```powershell
./scripts/build_miner_static.ps1
Desde Linux/Mac:
code
Bash
./scripts/build_miner_static.sh
El archivo resultante en dist/target/x86_64-unknown-linux-musl/release/miner-worker debe subirse a la nube y su URL ponerse en el secreto MINER_BINARY_URL.
3. Lanzamiento del Enjambre
Opción A: Automática (Cron)
El flujo está programado para ejecutarse cada 6 horas (.github/workflows/provisioner-cron.yml).
Opción B: Manual (Panic Button)
Ve a la pestaña Actions en GitHub.
Selecciona el workflow Hydra-Zero Swarm Launch.
Haz clic en Run workflow.
Configura:
Workers por Runner: 30 (Recomendado).
Shards: 5 (Total 150 nodos).
4. Monitoreo
Accede a tu Dashboard en Vercel (/dashboard).
Fleet Grid: Verás las capturas de pantalla de los nodos inicializando.
Identity Vault: Verifica si alguna cuenta ha sido revocada automáticamente.
Status: OPERATIONAL

---

MANUAL DE PREREQUISITOS DE DESPLIEGUE (CHECKLIST HUMANO)
Antes de que siquiera intentes hacer git push, verifica esto. Si falta algo, el "Pre-Flight Check" del código fallará.
A. BACKEND (Render)
Deploy Inicial: Render compilará el Dockerfile. Puede fallar la primera vez si las variables no están.
Environment Variables (Requeridas):
DATABASE_URL: libsql://[tu-db].turso.io
TURSO_AUTH_TOKEN: Token de Turso.
WORKER_AUTH_TOKEN: Genera uno fuerte (ej: openssl rand -hex 32).
RUST_LOG: info.
B. FRONTEND (Vercel)
Environment Variables (Requeridas):
NEXT_PUBLIC_API_URL: La URL que te dio Render (ej: https://prospector.onrender.com/api/v1).
NEXT_PUBLIC_API_TOKEN: Mismo que WORKER_AUTH_TOKEN (o uno específico de admin si implementas roles).
NEXT_PUBLIC_ADMIN_PASSWORD: Contraseña para el AdminGuard (Login local del dashboard).
CRÍTICO PARA C2:
GITHUB_PAT: Token personal de GitHub (Classic) con scopes repo y workflow.
GITHUB_OWNER: Tu nombre de usuario de GitHub.
GITHUB_REPO: El nombre de este repositorio (prospector-btc).
AUTH_SECRET: Generado con npx auth secret.
AUTH_GOOGLE_ID / AUTH_GOOGLE_SECRET: Credenciales OAuth de Google Cloud Console (para el login del Dashboard).
C. PROVISIONER (GitHub Actions Secrets)
Ve a Settings > Secrets and variables > Actions en tu repo.
ORCHESTRATOR_URL: La URL de Render.
WORKER_AUTH_TOKEN: El mismo token compartido.
MINER_BINARY_URL: URL directa al release de GitHub del binario miner-worker compilado estáticamente.
GOOGLE_COOKIES_JSON: (Opcional) Array JSON de cookies para que los workers no inicien como anónimos.
4. VALIDACIÓN DE CUENTAS (LOGICA DE NEGOCIO)
Para cumplir con "verificar que no sean las mismas", el Backend (IdentityRepository) ya tiene un ON CONFLICT DO UPDATE.
Sin embargo, para evitar usar la misma cuenta en múltiples workers simultáneamente (lo que garantiza un ban inmediato), el Orquestador tiene una lógica de Lease (Arrendamiento).
Confirmación de Lógica (Ya implementada en libs/infra/db-turso):
Cuando un worker pide identidad (/identities/lease), la base de datos marca esa identidad con un timestamp leased_until.
Si otro worker pide identidad, la query SQL (LEASE_ACTIVE_IDENTITY) ignora las que están "leased" o "revoked".
Resultado: Es matemáticamente imposible que dos workers reciban la misma cookie al mismo tiempo, cumpliendo tu requisito de seguridad.
🏁 ORDEN DE EJECUCIÓN
Configura las variables en Vercel y Render (Checklist arriba).
Sube el código (git push).
Entra al Dashboard (Vercel).
Ve a Identity Vault e inyecta tus cookies (formato JSON array).
Ve a Command & Control, selecciona 5 Workers / 1 Shard.
Pulsa Initialize.
Verás el Pre-Flight Modal.
Si sale ✅ en todo, pulsa IGNITE.
Si sale ❌ en "Identity Vault Capacity", significa que necesitas inyectar más cookies o bajar la cantidad de workers.

---


