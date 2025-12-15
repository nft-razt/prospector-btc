INFORME DE ARQUITECTURA: PROSPECTOR SYSTEM
Estrategia: Monolito Modular Políglota (Rust + TypeScript)
Gestor: Nx (Native Execution)

1. Evaluación y Veredicto Arquitectónico
   ¿Por qué NO DDD Puro o Hexagonal Clásico?
   DDD Puro: Es excelente para reglas de negocio complejas empresariales, pero Prospector es un sistema de alto cómputo. Sobrecargar el código Rust con capas de "Application Services" y "Domain Events" excesivos matará el rendimiento (latencia).
   Hexagonal (Ports & Adapters): Es vital para desacoplar la base de datos (Turso), pero implementarla rígidamente en cada módulo pequeño genera demasiado boilerplate (código repetitivo).
   La Solución Ganadora: Clean Modular Monolith
   Usaremos Nx para imponer límites físicos entre módulos. Cada módulo será una "caja negra" especializada (Plug & Play).
   Principio de Inversión de Dependencia (SOLID): El núcleo matemático (Rust) NO sabrá que existe Turso o Google Colab. Solo expondrá interfaces.
   Escalabilidad Horizontal: Si mañana quieres cambiar Turso por PostgreSQL, solo cambias el módulo de infraestructura. El núcleo no se toca.
   Performance (Rust): Los módulos críticos serán no_std (sin basura del sistema operativo) para máxima velocidad.
2. Nx vs. Workspaces Simples
   Ganador Indiscutible: Nx
   Usar solo workspaces (npm/cargo) es insuficiente.
   Computation Caching: Nx sabe qué has tocado. Si solo modificas el CSS del Dashboard, Nx NO recompilará el binario de Rust del Minero. Esto ahorra horas de compilación.
   Module Boundaries: Nx nos permite configurar reglas de ESLint/Linter que prohíben arquitectónicamente que el Frontend importe código directo de la Base de Datos. Esto es vital para la integridad a largo plazo.
   Graph Visualization: Podrás ver gráficamente cómo tus 300 nodos dependen del núcleo matemático.
3. El Plano Maestro de Directorios (The Elite Structure)
   Esta estructura respeta los alias @prospector/_ y separa responsabilidades por capas de abstracción.
   code
   Text
   prospector/
   ├── .nx/ # Cerebro de Nx (Caché y Grafo)
   ├── apps/ # APLICACIONES (Puntos de entrada ejecutables)
   │ ├── orchestrator/ # (Rust) API Server (Koyeb) - El Comandante
   │ ├── miner-worker/ # (Rust) Binario estático (Colab/GPU) - El Soldado
   │ ├── web-dashboard/ # (Next.js) El Observatorio Público
   │ ├── cli-admin/ # (Rust) Herramienta de gestión y carga masiva
   │ └── census-taker/ # (Rust) ETL para procesar BigQuery -> Turso
   │
   ├── libs/ # BIBLIOTECAS (Bloques LEGO reutilizables)
   │ │
   │ ├── core/ # [CAPA 1: DOMINIO PURO] (Rust)
   │ │ ├── crypto-math/ # (@prospector/core-math) Curva Elíptica, SIMD, Aritmética.
   │ │ ├── wallet-gen/ # (@prospector/wallet-gen) Lógica de generación (BIP32/Legacy).
   │ │ └── bloom-filter/ # (@prospector/bloom) Implementación probabilística optimizada.
   │ │
   │ ├── domain/ # [CAPA 2: LÓGICA DE NEGOCIO] (Rust/TS)
   │ │ ├── strategy-brain/ # (@prospector/strategy) Diccionarios de ataque, patrones humanos.
   │ │ └── models/ # (@prospector/models) Tipos compartidos (Structs, DTOs).
   │ │
   │ ├── infrastructure/ # [CAPA 3: ADAPTADORES] (Rust/TS)
   │ │ ├── db-turso/ # (@prospector/db) Cliente Turso/libSQL y Schemas SQL.
   │ │ ├── google-bq/ # (@prospector/google-bq) Queries y cliente BigQuery.
   │ │ └── comms-proto/ # (@prospector/proto) Serialización (Cap'n Proto/Bincode).
   │ │
   │ ├── features/ # [CAPA 4: UI FEATURES] (React/Next.js)
   │ │ ├── rich-list-ui/ # (@prospector/ui-richlist) Gráficas D3 de ballenas.
   │ │ ├── zombie-check/ # (@prospector/ui-zombie) Buscador de wallets perdidas.
   │ │ └── network-map/ # (@prospector/ui-map) Mapa de nodos activos.
   │ │
   │ └── shared/ # [CAPA 5: UTILIDADES] (Agnóstico)
   │ ├── ui-kit/ # (@prospector/ui-kit) Botones, Inputs, Layouts (Shadcn/Tremor).
   │ └── utils/ # (@prospector/utils) Loggers, Formatters.
   │
   ├── tools/ # SCRIPTS DE SOPORTE
   │ ├── provisioner/ # Scripts de Puppeteer para despertar Colabs.
   │ └── docker/ # Dockerfiles optimizados (Multi-stage).
   │
   ├── Cargo.toml # Workspace Rust (Raíz)
   ├── nx.json # Reglas de juego Nx
   └── tsconfig.base.json # Alias TypeScript (@prospector/_)
4. Reglas de Juego (Boundaries & Constraints)
   Para mantener la arquitectura "Soberana", definiremos reglas estrictas en nx.json o .eslintrc.
   Las 4 Leyes de Prospector:
   Ley de Gravedad: apps pueden importar libs, pero libs NUNCA pueden importar apps.
   Ley de Pureza: libs/core (Matemáticas) NO puede importar libs/infrastructure. El núcleo matemático no debe saber qué es una base de datos.
   Ley de Lenguaje:
   libs/core es territorio Rust exclusivo.
   libs/features es territorio TypeScript exclusivo.
   La comunicación entre ambos mundos se hace vía WASM (si corre en navegador) o API REST/Socket (si corre en servidor).
   Ley DRY (Don't Repeat Yourself): Si una lógica se usa en el miner y en el cli-admin, DEBE extraerse a una lib en domain.
5. Implementación de los Alias (tsconfig y Cargo.toml)
   Para que la experiencia de desarrollo sea "Elite", configuraremos los paths para que las importaciones sean semánticas.
   En TypeScript (tsconfig.base.json):
   code
   JSON
   {
   "compilerOptions": {
   "paths": {
   "@prospector/ui-kit": ["libs/shared/ui-kit/src/index.ts"],
   "@prospector/db": ["libs/infrastructure/db-turso/src/index.ts"],
   "@prospector/ui-richlist": ["libs/features/rich-list-ui/src/index.ts"]
   }
   }
   }
   En Rust (Cargo.toml raíz):
   code
   Toml
   [workspace]
   members = ["apps/*", "libs/*"]

[workspace.dependencies]

# Alias internos para Rust

prospector-core-math = { path = "libs/core/crypto-math" }
prospector-db = { path = "libs/infrastructure/db-turso" }
prospector-proto = { path = "libs/infrastructure/comms-proto" }
Uso en Código Rust:
code
Rust
// apps/miner-worker/src/main.rs
use prospector_core_math::secp256k1::Generator; // ¡Limpio y Profesional! 6. Justificación de Escalabilidad y Futuro
Plug and Play: Si mañana quieres añadir soporte para Ethereum, creas libs/core/eth-math y libs/domain/strategy-eth. El resto de la infraestructura (Orquestador, DB, Dashboard) se reutiliza al 100%.
Trabajo en Equipo: Un experto en React puede trabajar en libs/features sin tocar (ni romper) el código Rust del core. Nx gestiona esto.
Despliegue Atómico: El miner-worker es un binario estático. No arrastra dependencias de Node.js ni UI. Pesa 10MB y corre en cualquier Linux.
Esta estructura es Definitiva. Cumple con la visión hiper-holística, separa las preocupaciones matemáticamente y prepara el terreno para una Tesis Doctoral indiscutible.

---
