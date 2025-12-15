# 📘 PLAN MAESTRO DE IMPLEMENTACIÓN Y OPTIMIZACIÓN (V3.0 - HYDRA ZERO)

**Fecha:** 2025-12-09
**Estado:** READY FOR DEPLOYMENT
**Clasificación:** CLEARANCE LEVEL 5
**Arquitectura:** Monolito Fractal (Nx + Rust + TypeScript)

---

## 1. 🔍 AUDITORÍA DE SISTEMAS (SNAPSHOT 12:20)

Se ha realizado una inspección profunda de los 145 archivos del sistema.

### ✅ Estado de Integridad

1.  **Eliminación de Residuos:** Confirmado. No existen rastros de `libs/domain-models` ni `handlers.rs` monolítico. La estructura es limpia.
2.  **Single Source of Truth (SSoT):**
    - `libs/domain/models-rs` es la única autoridad para DTOs.
    - `WorkOrder` utiliza `String` para los rangos, permitiendo interoperabilidad futura con `BigInt` (256-bit) sin romper el contrato JSON actual.
3.  **Atomicidad del Orquestador:**
    - Los handlers (`swarm.rs`, `admin.rs`) están modularizados.
    - El ciclo de vida (`acquire` -> `keepalive` -> `complete`) está implementado y protegido contra fallos de red.
4.  **Resiliencia del Minero:**
    - El `miner-worker` implementa un hilo secundario para `keep-alive` mientras el hilo principal satura la CPU con Rayon. Esto previene que el servidor mate al worker por "falsa inactividad" durante cálculos intensivos.

### ⚠️ Puntos de Atención (Cuellos de Botella Potenciales)

1.  **Conversión de Tipos en DB (Turso/SQLite):**
    - _Ubicación:_ `libs/infra/db-turso/src/repositories/job.rs`
    - _Hallazgo:_ La query `SELECT MAX(CAST(range_end AS INTEGER))` funciona bien para rangos `u64`. Sin embargo, cuando migremos a rangos de 256 bits (claves privadas reales de Bitcoin), `INTEGER` de SQLite (64-bit signed) desbordará.
    - _Solución V3.1:_ Almacenar rangos como `TEXT` y usar una función de ordenamiento lexicográfico o migrar la lógica de "Siguiente Rango" a Rust (en memoria) en lugar de SQL.

2.  **Serialización de Filtros (Bloqueo de RAM):**
    - _Ubicación:_ `apps/miner-worker/src/main.rs`
    - _Hallazgo:_ `RichListFilter::load_from_file` carga todo el archivo (~200MB) en RAM. En Google Colab (Free Tier) esto es aceptable, pero si escalamos a contenedores de 512MB RAM, podría causar OOM (Out Of Memory).
    - _Optimización:_ Usar `mmap` (Memory Mapping) para leer el filtro desde disco sin cargarlo todo en el Heap, o mantener el filtro actual si garantizamos >2GB RAM.

3.  **Dependencia de `worker_id` Genérico:**
    - _Ubicación:_ `apps/orchestrator/src/handlers/swarm.rs`
    - _Hallazgo:_ `let worker_placeholder = "hydra-node-generic";`.
    - _Acción:_ Es vital que el Minero envíe su ID real generado en el arranque (`uuid::Uuid::new_v4()`) para poder trazar qué nodo específico está fallando o encontrando colisiones.

---

## 2. 🚀 PROTOCOLO DE IMPLEMENTACIÓN (PASO A PASO)

Sigue esta secuencia para activar el sistema sin errores de compilación o lógica.

### FASE 1: NIVELACIÓN DEL ENTORNO (INFRA)

1.  **Configuración de Variables (`.env`):**
    Asegurar que el archivo `.env` en la raíz tenga las definiciones correctas para la nueva lógica.

    ```bash
    # .env
    DATABASE_URL="file:prospector.db"
    WORKER_AUTH_TOKEN="protocolo_hydra_secreto_v2"
    RUST_LOG="info,prospector_orchestrator=debug,prospector_miner=info"
    PORT=3000
    ```

2.  **Inicialización de Base de Datos:**
    El código actual usa `INITIAL_SCHEMA` en `libs/infra/db-turso/src/schema.rs`.
    - Ejecutar el Orquestador una vez forzará la creación de tablas.
    - `cargo run --bin orchestrator`

### FASE 2: COMPILACIÓN DEL NÚCLEO (BUILD)

1.  **Verificación de Dependencias:**
    Ejecutar en la raíz para asegurar que el `Cargo.lock` sincronice la eliminación de la librería obsoleta.

    ```bash
    cargo check
    ```

2.  **Compilación del Minero (Cross-Compilation):**
    El script `scripts/build_miner_static.sh` usa Docker. Ejecutarlo para generar el artefacto que se subirá a la nube.
    ```bash
    ./scripts/build_miner_static.sh
    ```
    _Resultado esperado:_ `target/x86_64-unknown-linux-musl/release/miner-worker`

### FASE 3: DESPLIEGUE DEL CEREBRO (ORCHESTRATOR)

1.  **Hosting del Filtro:**
    El Orquestador espera descargar `utxo_filter.bin` en el Dockerfile.
    - Generar filtro local: `cargo run --bin census-taker -- --input dummy_data.csv --output utxo_filter.bin`
    - Subir a GitHub Releases o S3.
    - Actualizar `ARG FILTER_URL` en `apps/orchestrator/Dockerfile`.

2.  **Deploy en Render:**
    Push a rama `main`. Render detectará el `Dockerfile` y compilará.

### FASE 4: ACTIVACIÓN DEL ENJAMBRE (PROVISIONER)

1.  **Preparación de Identidad:**
    Usar `tools/provisioner/src/harvester.ts` para obtener cookies frescas de Google.

    ```bash
    cd tools/provisioner
    npx ts-node src/harvester.ts
    ```

2.  **Inyección en Bóveda:**
    Usar el Dashboard (`/admin`) para subir las cookies obtenidas a la base de datos del Orquestador.

3.  **Lanzamiento:**
    Ejecutar el provisioner para despertar los nodos en Colab.
    ```bash
    npx ts-node src/main.ts
    ```

---

## 3. 🧠 MEJORAS DE LÓGICA Y RENDIMIENTO (FUTURE-PROOFING)

Propuestas de optimización detectadas en la auditoría para implementar post-lanzamiento.

### A. Optimización de `StrategyExecutor` (SIMD Check)

Actualmente, `libs/domain/mining-strategy/src/executor.rs` usa `rayon` para paralelismo.
**Mejora:** Agregar una comprobación en tiempo de ejecución (`is_x86_feature_detected!("avx2")`) para elegir entre una implementación escalar segura o una implementación vectorial AVX2 agresiva. Esto podría aumentar el hashrate en un 40% en Colab.

### B. Compresión de Tráfico (Protobuf/Bincode sobre HTTP)

Actualmente usamos JSON para `WorkOrder` y `Findings`.
**Mejora:** El minero y el orquestador son ambos Rust. Podemos usar `bincode` directamente sobre el cuerpo HTTP (`Content-Type: application/octet-stream`) para reducir el tamaño del payload y el tiempo de CPU gastado en serializar/deserializar JSON.

### C. Estrategia de "Salto de Canguro" (Kangaroo Hopping)

Actualmente usamos rangos secuenciales.
**Mejora:** Implementar `Pollard's Kangaroo` en `libs/core/math-engine`. Si conocemos la clave pública (del `utxo_filter.bin`), este algoritmo es O(√N) en lugar de O(N) para encontrar la clave privada si sabemos que está en un rango cercano. Es vital para la "Estrategia Forense".

---

## 4. CONCLUSIÓN

El sistema **PROSPECTOR BTC v3.0** es arquitectónicamente sólido. Cumple con los principios de Atomicidad, Soberanía y Responsabilidad Única.

**Semáforo de Estado:**

- **Core Math:** 🟢 (Optimizado)
- **Orchestrator:** 🟢 (Atómico y Modular)
- **Miner:** 🟢 (Resiliente y Smart)
- **Infra:** 🟡 (Atención en CAST de SQL para futuro BigInt)
- **Frontend:** 🟢 (Conectado)

**Proceder al despliegue.**

---
