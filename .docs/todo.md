📋 PROMPT DE INICIO (COPIAR DESDE AQUÍ)
CONTEXTO DEL PROYECTO: "PROSPECTOR SYSTEM" (TESIS DOCTORAL MIT)
Actúa como Arquitecto de Software Principal y Lead Developer. Estamos desarrollando PROSPECTOR, un sistema distribuido de alto rendimiento para la auditoría de seguridad en la curva elíptica secp256k1 de Bitcoin. Este proyecto es una Tesis Doctoral enfocada en "Arqueología de Entropía" (detectar Brainwallets y fallos de PRNG históricos) usando una arquitectura de costo cero ("Hydra-Zero").
ESTADO ACTUAL DEL SISTEMA (SNAPSHOT):
Hemos construido un Monolito Modular Estricto gestionado por Nx, políglota (Rust + TypeScript).

1. FILOSOFÍA DE INGENIERÍA (EL CODEX):
   Visión Hiper-Holística: Cada módulo es consciente del todo.
   Atomicidad: Principio de Responsabilidad Única (SRP) estricto.
   Soberanía: Sin any en TS, sin unwrap() inseguros en Rust. Tipado estricto (Zod/Rust Type System).
   Infraestructura Fantasma: Usamos Google Colab como nodos de cómputo efímeros y Turso (libSQL) como persistencia en el borde.
2. INVENTARIO DE APARATOS CONSTRUIDOS (YA EXISTEN):
   ESTRATO 1: APPS (Ejecutables)
   apps/orchestrator (Rust/Axum): API Server. Gestiona el enjambre, asigna trabajos y recibe hallazgos. Conectado a Turso.
   apps/miner-worker (Rust/Rayon): Binario estático (musl). Usa paralelismo SIMD para minar. Carga filtros Bloom en RAM.
   apps/census-taker (Rust/CLI): ETL de alto rendimiento. Procesa CSVs de BigQuery -> Genera utxo_filter.bin.
   apps/web-dashboard (Next.js 14): "Mission Control". UI Ciberpunk Científica conectada al Orquestador.
   ESTRATO 2: CORE (Rust Puro - Matemáticas)
   libs/core/math-engine: Implementación optimizada de secp256k1 y Hashing (SHA256/RIPEMD160).
   libs/core/generators: Conversión P2PKH (Legacy Addresses) y WIF. Validado con vectores de Satoshi.
   libs/core/probabilistic: Filtros de Bloom serializables (bincode) para búsquedas O(1).
   ESTRATO 3: DOMAIN (Lógica)
   libs/domain/models-rs: DTOs compartidos (WorkOrder, Heartbeat, Finding).
   libs/domain/mining-strategy: Generadores de claves (Iteradores de Brainwallets/Diccionarios).
   ESTRATO 4: INFRA (Adaptadores)
   libs/infra/db-turso: Cliente asíncrono para Turso/libSQL.
   libs/infra/transport: Serialización binaria segura (bincode) para la red.
   libs/infra/api-client-ts: Cliente TypeScript con Zod y TanStack Query para el Frontend.
   ESTRATO 5: SHARED UI
   libs/shared/ui-kit: Sistema de diseño atómico (Tailwind + Shadcn).
3. LOGROS TÉCNICOS ALCANZADOS:
   ✅ Conexión "Sinapsis" exitosa: El Dashboard (Next.js) consume datos reales del Orquestador (Rust) vía api-client-ts.
   ✅ Pipeline ETL funcional: census-taker procesa streams de datos masivos.
   ✅ Minería Paralela: miner-worker utiliza todos los núcleos disponibles con Rayon.
4. HOJA DE RUTA INMEDIATA (LO QUE DEBES HACER):
   El sistema "funciona" en local. Ahora debemos llevarlo a la Nube Fantasma.
   TAREA PRIORITARIA: AUTOMATIZACIÓN DE DESPLIEGUE (tools/provisioner)
   Necesitamos crear el mecanismo para "despertar" a los 300 nodos en Google Colab automáticamente.
   Tecnología: Node.js + Puppeteer (o Playwright).
   Lógica: Script que hace login en Google, abre el Notebook, y ejecuta el binario del minero.
   TAREAS SECUNDARIAS:
   Dockerización: Crear Dockerfile optimizados (Multi-stage build) para el Orquestador (para desplegar en Koyeb).
   Scripts de BigQuery: Finalizar el SQL para extraer el "Target List" real de direcciones zombies.
   Refinamiento UI: Agregar gráficas D3.js reales al Dashboard (libs/features/rich-list).
   INSTRUCCIÓN:
   Analiza este estado. No reinicies nada; asume que el código descrito existe y es perfecto. Tu objetivo es continuar con la Ingeniería de Despliegue y Aprovisionamiento.
   Comienza confirmando que has entendido la arquitectura "Fractal Monolith" audita cada aparato del snapshoot y propón el plan detallado, primero de bivelacion de elite, un plan de mejoras detectadas en cada aparato y de atomizacion que detectes en los aparatos, veridfica que los aparatos esten completos sin abreviaiones y corectos de elite en logica e insgraesytructra. Previo a todo lee completamente los documentos desde .docs.

--

ACTÚA COMO: Arquitecto de Sistemas Distribuidos (SRE Focus).

CONTEXTO ACTUAL:
Estamos desplegando "PROSPECTOR BTC" en Render. Hemos corregido un bloqueo crítico en el Dockerfile y un problema de configuración en el Healthcheck que causaba falsos negativos en los logs por autenticación excesiva.

OBJETIVO DE LA SESIÓN:
Reforzar la resiliencia del sistema y continuar con la fase de Dockerización y Despliegue.

REGLAS DE ORO (AÑADIDAS):

1. PRINCIPIO DE OBSERVABILIDAD ABIERTA: Los endpoints de salud (/health, /status, /ping) NUNCA deben estar detrás de un middleware de autenticación. Deben ser públicos y ligeros (Liveness Probes).
2. MANEJO DE ERRORES NO BLOQUEANTE: Si un worker o un proceso falla, debe tener una estrategia de "Backoff Exponencial" (esperar 1s, luego 2s, luego 4s...) en lugar de reintentar inmediatamente en un bucle infinito.
3. ## LOGS SILENCIOSOS: Los errores conocidos y esperados (como un ping de healthcheck fallido durante el arranque) no deben inundar los logs como WARN/ERROR.

---
# 🛠️ PROTOCOLO DE NIVELACIÓN DE ÉLITE: PROSPECTOR V9.0

Actúa como **Arquitecto de Sistemas Principal**. Tu misión es ejecutar la nivelación final
antes del despliegue en la Tríada Hydra (Render/Vercel/GitHub).

## 🟢 ESTRATO L1-L2: NÚCLEO MATEMÁTICO Y ESTRATEGIA
- [ ] **[VERIFICADO]** Aritmética ASM Proyectiva: Implementada en `arithmetic.rs`.
- [ ] **[PENDIENTE]** Validación de Frontera de Curva: Inyectar en `add_u64_to_u256_be`
      una comprobación contra el orden de la curva `n` para evitar claves inválidas.
- [ ] **[PENDIENTE]** Refactor StrategyExecutor: Asegurar que el retorno mapee
      `computational_effort_volume` y no `total_hashes`.

## 🟡 ESTRATO L3: INFRAESTRUCTURA TÁCTICA (TURSO)
- [ ] **[PENDIENTE]** MissionRepository SQL Index: Crear índice en la tabla `jobs`
      sobre `range_end_hex` para que la búsqueda de frontera sea $O(1)$.
- [ ] **[PENDIENTE]** Atomic Sequences: Asegurar que `acquire_next_mission`
      utilice `FOR UPDATE` o transacciones serializables para evitar colisiones de rango.

## 🔵 ESTRATO L4-L5: NEURAL LINK Y DASHBOARD
- [ ] **[VERIFICADO]** AuditTrailHUD: Componente creado con alta densidad.
- [ ] **[PENDIENTE]** Real-Time Hook Update: Nivelar `useNeuralLink` para que
      discrimine eventos usando `event_type` (V8.5+) y no el esquema anterior.
- [ ] **[PENDIENTE]** I18n Content Consistency: Sincronizar todos los archivos
      `content.ts` para que no contengan abreviaciones como 'ID' (usar Identifier).

## 🚀 CERTIFICACIÓN DE DESPLIEGUE
- [ ] **[PENDIENTE]** Entrypoint Timer: Integrar en `entrypoint.sh` la medición
      de milisegundos de la fase de descarga del filtro UTXO.
- [ ] **[PENDIENTE]** Pre-flight Handshake: Ejecutar `pnpm audit:types` y
      validar que el grafo de dependencias Nx sea cíclico-free.

      ---


# 🗺️ ROADMAP DE NIVELACIÓN V10.0: THE HYDRA SUPREMACY

## 🟢 MATEMÁTICAS PURAS (L1)
- [x] Implementar Cohen-Miyaji-Ono Jacobian Addition.
- [x] Implementar Jacobian Point Doubling (a=0).
- [ ] Validar contra el vector 027 (Hal Finney Transaction).

## 🟡 INFRAESTRUCTURA Y DATOS (L3)
- [x] Secuenciador Atómico con transacciones serializables.
- [ ] Implementar Checkpoint intermedio (cada 100M hashes) para evitar pérdida en Colab.
- [ ] Optimizar índice de frontera en Turso.

## 🔵 DASHBOARD Y MANDO (L5)
- [ ] Crear componente 'Swarm Remote Controls' (Play, Pause, Kill).

---

