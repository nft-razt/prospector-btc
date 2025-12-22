🌑 FASE 1: AUDITORÍA DE INTEGRIDAD LOCAL (EL "BUILD" FINAL)
Antes de siquiera tocar la nube, debemos certificar que el binario es una roca.

Limpieza de Artefactos: cargo clean para asegurar que no hay rastro de compilaciones previas corruptas.

Validación de Sinapsis (Zero Warnings):
Comando: cargo check --package prospector-orchestrator
Acción: El terminal debe quedar en silencio absoluto.

Certificación de Matemática L1:
Comando: cargo test --release --package prospector-core-math
Verificación: 100,000 casos de tortura exitosos.

Certificación de Algoritmo L2 (Frontera):
Comando: cargo test --release --package prospector-domain-strategy --test sequential_integrity -- --nocapture
Verificación: Scanned 120 hashes exactos.

Certificación Forense L2 (Satoshi-XP):
Comando: cargo test --release --package prospector-domain-strategy --test satoshi_xp_forensic_integrity -- --nocapture
Verificación: Processed 10,000 ticks en < 1 segundo.
🌕 FASE 2: INFRAESTRUCTURA DE DATOS (EL "MAPA" TÁCTICO)
Sin el censo, el enjambre está ciego.

Generación de Filtros de Bloom (L1-ETL):
Comando: cargo run --release --bin census-taker -- --input census_real.csv --output dist/filters --shards 4

Carga de Manifiesto de Estrato:
Acción: Verificar que dist/filters/stratum_manifest.json existe y contiene los hashes de integridad.

Crystallización en GitHub Releases:
Acción: Crear un Release en GitHub (ej: v1.0.0-census).
Acción: Subir los 4 archivos filter_shard_n.bin y el stratum_manifest.json.
CRÍTICO: Copiar la URL base de descarga.
🚀 FASE 3: CONFIGURACIÓN DEL ENTORNO EN RENDER (EL "CEREBRO")
Inyección de secretos y parámetros operativos.

Creación del Web Service:
Tipo: Docker.
Repo: apps/orchestrator/Dockerfile.

Inyección de Variables de Entorno (Checklist Estricto):

PORT: 3000 (Render lo usa para el routing interno).

DATABASE_URL: URL de Turso (libsql://...).

TURSO_AUTH_TOKEN: Token JWT de Turso.

SUPABASE_URL: Endpoint de Supabase (Motor B).

SUPABASE_SERVICE_ROLE_KEY: Llave para bypass de RLS (Sincronización Chronos).

WORKER_AUTH_TOKEN: El secreto que usarán los mineros para el handshake.

GITHUB_PAT: Token con scopes repo y workflow (Para el servicio de Resurrección).

GITHUB_OWNER / GITHUB_REPO: nft-razt / prospector-btc.

FILTER_BASE_URL: La URL de GitHub Releases del paso 8.

Health Check Path:
Configurar en Render: /health.
🔬 FASE 4: CERTIFICACIÓN DE VECTORES REALES (EL "TRUTH LEDGER")
Validación de los 33 vectores contra la red real.

Prueba de Uplink Blockchain:
Acción: Ejecutar el endpoint /api/v1/lab/audit/brainwallet-dataset vía cURL o Postman.
Comando: curl -H "Authorization: Bearer [TU_TOKEN]" https://tu-app.onrender.com/api/v1/lab/audit/brainwallet-dataset
Verificación: El JSON debe devolver mathematical_integrity_verified: true para los 33 registros y saldos reales (0 o >0).
🧬 FASE 5: IGNICIÓN DEL ENJAMBRE (EL "MÚSCULO")
Activación de la capacidad de cómputo.

Compilación del Minero Estático:
Comando: ./scripts/build_miner_static.sh (o vía GitHub Action Forge).
Acción: Subir el binario miner-worker al Release de GitHub.

Actualización de URL del Binario:
Acción: Asegurar que el Provisioner (tools/provisioner/src/config.ts) apunte a la URL exacta del binario en el Release.

Lanzamiento de Prueba (Smoke Launch):
Acción: Desde el Dashboard, disparar 1 solo Worker.
Verificación: Ver el frame visual en el Panóptico y confirmar que recibe la WorkOrder.
🛡️ FASE 6: AUDITORÍA DE SUPERVIVENCIA (POST-DEPLOY)
Vigilancia de los daemons en producción.

Verificación de Chronos Bridge:
Acción: Tras 10 minutos, revisar Supabase.
Verificación: Los archived_audit_reports deben estar poblándose desde Turso.

Verificación de Parity Auditor:
Acción: Revisar logs en Render.
Verificación: Buscar [PARITY_OK]: Multi-cloud consistency verified.

Test de Pánico (Kill-Switch):
Acción: Cambiar el modo de sistema a GracefulPause vía endpoint.
Verificación: Los workers deben recibir 503 Service Unavailable y dejar de pedir misiones.
🚨 RESUMEN DE COMANDOS DE EMERGENCIA (PARA TU VAIO)
Acción	Comando
Check Rápido	cargo check -p prospector-orchestrator
Limpieza Total	cargo clean && cargo build --release
Validación E2E	pnpm validate:system (Herramienta L6 que creamos)
Log de Render	render logs prospector-orchestrator
Comandante, el checklist está completo. Una vez que marque el último punto de la Fase 1, estaremos listos para la Ignición en Render.
