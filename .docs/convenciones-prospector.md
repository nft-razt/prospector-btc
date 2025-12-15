CODEX RAZSMART: CONSTITUCIÓN DE INGENIERÍA DE SOFTWARE SOBERANA v3.0
Clasificación: DOCTRINA FUNDAMENTAL
Alcance: Global (Monorepo NX)
Estado: IMPERATIVO
ÍNDICE MAESTRO
Filosofía Raíz: El Camino del Ingeniero Soberano
Arquitectura de Aparatos (The Fractal Monolith)
Los 12 Pilares de Integridad del Código (La Ley)
Estándares de Tipado Estricto y Zod (SSoT)
Gestión de Estado: La Doctrina Zustand
Protocolo de UI/UX: Internacionalización y Diseño Atómico
Protocolo de Refactorización Holística
Convenciones de Nomenclatura y Estructura de Archivos
Flujo de Trabajo y Git (Commit Semántico)
Directrices de Interacción con la IA (Meta-Prompt)

1. FILOSOFÍA RAÍZ: EL CAMINO DEL INGENIERO SOBERANO
   1.1. Visión Holística
   No escribimos "scripts"; forjamos Aparatos. Un Aparato es una entidad de software autónoma, resiliente y observable. Nunca se escribe una línea de código sin entender su impacto en la totalidad del ecosistema. La arquitectura es fractal: la calidad del sistema completo depende de la perfección de su función más pequeña.
   1.2. Cero Deuda Técnica (Zero Regressions)
   La velocidad nunca justifica la suciedad. Si una refactorización rompe un contrato, se detiene, se analiza y se corrige el consumidor antes de proceder. El código "legacy" no existe; solo existe código que aún no ha sido elevado al estándar del CODEX.
   1.3. Atomicidad y Responsabilidad Única (SRP)
   Cada Aparato (librería, componente, hook, utilidad) debe hacer una sola cosa y hacerla de manera sublime. Si un componente necesita "y" para describir su función (ej: "Muestra la tabla Y filtra los datos"), debe ser dividido.
2. ARQUITECTURA DE APARATOS (THE FRACTAL MONOLITH)
   Utilizamos Nx para orquestar un monorepo modular. Cada pieza de lógica reside en su propia librería (libs), categorizada estrictamente.
   2.1. Taxonomía de los Aparatos
   Tipo de Aparato Tag Nx (type:) Ubicación Canónica Responsabilidad Dependencias Permitidas
   Feature UI feature-ui libs/razsmart/features/ui/_ Componentes visuales complejos (Smart Components) conectados a estado y dominio. ui-kit, domain, data-access
   UI Kit ui-kit libs/razsmart/ui/_ Componentes visuales "tontos" (Dumb Components), atómicos y reutilizables. shared-types
   Domain Logic domain libs/razsmart/domain/_ Lógica pura de negocio, validaciones Zod, utilidades de cálculo. shared-types
   Data Access data-access libs/razsmart/data-access/_ Comunicación con DB (Prisma), APIs externas. shared-types, shared-util
   Shared Util util libs/shared/util/\* Helpers agnósticos al dominio (formatters, loggers). Ninguna (idealmente)
   2.2. Anatomía de un Aparato (Estructura de Carpetas)
   Cada librería generada debe seguir esta estructura interna rigurosa:
   code
   Text
   libs/razsmart/features/ui/dashboard-analytics/
   ├── src/
   │ ├── components/ # Sub-componentes internos (no exportados)
   │ ├── hooks/ # Hooks específicos de este feature
   │ ├── stores/ # Stores de Zustand locales (si aplica)
   │ ├── schemas/ # Schemas Zod de entrada/salida UI
   │ ├── i18n/ # JSONs de traducción locales
   │ ├── lib/ # Utilidades puras internas
   │ ├── index.ts # BARREL FILE: Único punto de exportación
   │ └── server.ts # (Opcional) Exportaciones exclusivas de servidor
   ├── **tests**/ # Tests unitarios e integración
   ├── README.md # Documentación del Aparato
   ├── project.json # Configuración Nx
   └── tsconfig.json # Configuración TS estricta
3. LOS 12 PILARES DE INTEGRIDAD DEL CÓDIGO (LA LEY)
   Contrato Zod Soberano: Toda frontera de datos (API, Formulario, URL Params) DEBE estar definida por un Schema Zod. TypeScript infiere sus tipos de Zod, nunca al revés.
   Inferencia de Tipos: Prohibido definir interfaces manuales si pueden inferirse (z.infer<typeof Schema>).
   Erradicación de any: El uso de any es motivo de rechazo inmediato. Usar unknown con Type Guards si la estructura es incierta.
   Validación de Entrada/Salida: Toda Server Action o API Route valida su entrada con Zod y devuelve un objeto tipado Result<T, E>.
   Shaping de Datos: Nunca exponer los modelos de base de datos crudos (Prisma) al cliente. Usar DTOs (Data Transfer Objects) transformados.
   Props Explícitas: Los componentes React definen sus props vía Zod o interfaces estrictas. Prohibido React.FC genérico sin definición.
   Hooks Tipados: Argumentos y retornos de hooks personalizados deben ser explícitos.
   Genéricos Reutilizables: Utilidades abstractas deben usar <T> para flexibilidad segura.
   Justificación de as: El casting (as Type) es un escape de emergencia. Debe llevar un comentario // SAFETY: ... explicando por qué es seguro.
   Configuración Segura: Variables de entorno validadas al inicio (env.mjs con Zod).
   Documentación TSDoc: Todo exportable debe tener JSDoc/TSDoc (@param, @returns, @example).
   Higiene Absoluta: Cero imports sin usar, cero console.log (usar Logger), cero código comentado.
4. ESTÁNDARES DE TIPADO ESTRICTO Y ZOD (SSoT)
   Zod es nuestra Fuente Única de Verdad (SSoT).
   4.1. Patrón de Definición de Entidades
   code
   TypeScript
   // ✅ CORRECTO
   import { z } from 'zod';

// 1. Definir Schema
export const UserSchema = z.object({
id: z.string().uuid(),
email: z.string().email(),
role: z.enum(['ADMIN', 'USER', 'GUEST']),
settings: z.object({
theme: z.enum(['light', 'dark']),
}).optional(),
});

// 2. Inferir Tipo
export type User = z.infer<typeof UserSchema>;

// 3. Crear Type Guard (Opcional pero recomendado)
export const isUser = (data: unknown): data is User => {
return UserSchema.safeParse(data).success;
};
4.2. Validación en Fronteras (Server Actions)
code
TypeScript
// ✅ CORRECTO
export async function updateUserAction(rawData: unknown) {
// 1. Validación Ofensiva
const result = UserSchema.safeParse(rawData);

if (!result.success) {
// Retorno tipado de error
return { success: false, errors: result.error.flatten() };
}

const data = result.data; // Aquí 'data' ya es de tipo 'User'
// ... lógica de negocio ...
} 5. GESTIÓN DE ESTADO: LA DOCTRINA ZUSTAND
Para el estado global o complejo del cliente, usamos Zustand. Evitamos Context API para estados de alta frecuencia para prevenir re-renders innecesarios.
5.1. Estructura del Store
Cada Aparato UI complejo puede tener su propio Store.
code
TypeScript
// libs/razsmart/features/ui/dashboard/stores/dashboard.store.ts
import { create } from 'zustand';
import { devtools } from 'zustand/middleware';

interface DashboardState {
isSidebarOpen: boolean;
selectedPeriod: 'day' | 'week' | 'month';
toggleSidebar: () => void;
setPeriod: (period: 'day' | 'week' | 'month') => void;
}

export const useDashboardStore = create<DashboardState>()(
devtools((set) => ({
isSidebarOpen: true,
selectedPeriod: 'week',
toggleSidebar: () => set((state) => ({ isSidebarOpen: !state.isSidebarOpen })),
setPeriod: (period) => set({ selectedPeriod: period }),
}))
);
5.2. Selectores Atómicos
Al consumir el store, seleccionar solo lo necesario.
code
TypeScript
// ✅ CORRECTO
const isOpen = useDashboardStore((state) => state.isSidebarOpen); 6. PROTOCOLO DE UI/UX: INTERNACIONALIZACIÓN Y DISEÑO ATÓMICO
6.1. Internacionalización (i18n) Granular
Cada Aparato debe ser auto-contenido, incluyendo sus textos.
Ubicación: libs/.../src/i18n/{es,en}.json
Consumo: Usando un hook custom o next-intl configurado para cargar namespaces dinámicos.
code
JSON
// libs/.../src/i18n/es.json
{
"filters": {
"search_placeholder": "Buscar por nombre...",
"export_btn": "Exportar CSV"
}
}
6.2. Componentes UI (Theming Semántico)
Prohibido usar colores hardcodeados (ej: bg-blue-500). Usar tokens semánticos definidos en tailwind.config.ts.
code
Tsx
// ❌ INCORRECTO

<div className="bg-red-500 text-white">Error</div>

// ✅ CORRECTO

<div className="bg-destructive text-destructive-foreground">Error</div>
7. PROTOCOLO DE REFACTORIZACIÓN HOLÍSTICA
Cuando solicites o realices una refactorización, se activa el siguiente protocolo:
Análisis de Impacto: Identificar todos los consumidores del componente/función a modificar (usar grafo de dependencias de Nx).
Modificación del Núcleo: Aplicar la mejora en el Aparato origen.
Propagación de Cambios: Actualizar todos los Aparatos dependientes para que cumplan con la nueva firma/contrato.
Limpieza: Eliminar código muerto resultante de la refactorización.
Verificación: Ejecutar tests de todos los Aparatos afectados (nx affected:test).
Nota: La IA debe entregar el código del Aparato modificado Y el código de los Aparatos dependientes actualizados.
8. CONVENCIONES DE NOMENCLATURA Y ESTRUCTURA
8.1. Archivos
Componentes: PascalCase.tsx (ej: UserProfile.tsx)
Hooks: camelCase.ts (ej: useAuth.ts)
Utilidades: kebab-case.ts (ej: date-formatter.ts)
Tests: [nombre].test.tsx o [nombre].spec.ts
8.2. Código
Interfaces/Tipos: PascalCase (ej: UserData). No usar prefijo I (ej: IUser ❌).
Variables Booleanas: Prefijos is, has, should (ej: isLoading, hasError).
Event Handlers: Prefijo handle (ej: handleSubmit).
Props de Eventos: Prefijo on (ej: onClick).
9. FLUJO DE TRABAJO Y GIT (COMMIT SEMÁNTICO)
Cada cambio debe ser atómico y seguir Conventional Commits:
feat(scope): Nueva funcionalidad.
fix(scope): Corrección de bug.
refactor(scope): Cambio de código que no altera funcionalidad.
docs(scope): Cambios en documentación.
test(scope): Añadir o corregir tests.
chore(scope): Configuración, build, dependencias.
Ejemplo: feat(ui-dashboard): implementar filtro por fechas con zod schema
10. DIRECTRICES DE INTERACCIÓN CON LA IA (META-PROMPT)
Este es el contrato que rige nuestra interacción. Al solicitar código:
Entregables Completos: Prohibido usar // ... resto del código o // igual que antes. El código se entrega funcional, completo y listo para Ctrl+C / Ctrl+V.
Contexto Explícito: La IA asumirá que está trabajando dentro de este Monorepo Nx con las tecnologías definidas.
Validación Cruzada: Si pido una UI, la IA debe generar automáticamente el Schema Zod correspondiente, los tipos TypeScript inferidos y el JSON de i18n base.
Heimdall (Logging): Todo código complejo debe incluir logs estructurados (logger.info, logger.error) por defecto.
Manejo de Errores: try/catch obligatorio en capas de datos, con mapeo a errores de dominio UI.

---

ACTUALIZACION 2025-06-12

Esta es la Arquitectura de Aparatos Definitiva (v3.0).
He aplicado una visión Hiper-Holística para asegurar que no falte ninguna pieza del rompecabezas. Hemos descompuesto el sistema "Prospector" bajo principios de Diseño Guiado por el Dominio (DDD) y Arquitectura Hexagonal, adaptados a un entorno políglota (Rust + TypeScript) gestionado por Nx.
Cada "Aparato" aquí listado tiene Responsabilidad Única (SRP), es Atómico, sigue el principio DRY (se escribe una vez, se usa en el orquestador, el minero y la CLI) y está preparado para escalar a futuros dominios (ej. Ethereum). 2. ARQUITECTURA DE APARATOS (THE FRACTAL MONOLITH)
Nomenclatura Soberana: @prospector/[capa]-[nombre]
La estructura se divide en 5 Estratos Geológicos (Capas). Las capas superiores pueden depender de las inferiores, pero nunca al revés (Principio de Dependencia Unidireccional).
ESTRATO 1: APPS (Puntos de Entrada)
Son los ejecutables. No contienen lógica de negocio compleja, solo orquestan los aparatos.
Aparato (App) Lenguaje Ubicación Tag Nx Descripción
orchestrator Rust apps/orchestrator type:app-api El Comandante. API Server (Axum) que asigna rangos y recibe hallazgos.
miner-worker Rust apps/miner-worker type:app-cli El Soldado. Binario estático compilado con MUSL/SIMD para Colab/GPU.
census-taker Rust apps/census-taker type:app-etl El Cartógrafo. ETL de alto rendimiento para procesar CSVs de BigQuery a Turso/Bloom.
cli-admin Rust apps/cli-admin type:app-cli Herramienta de gestión manual (generar filtros, seed database, health check).
web-dashboard Next.js apps/web-dashboard type:app-web El Observatorio. Interfaz pública reactiva para visualización de datos.
ESTRATO 2: CORE (El Núcleo Matemático - Rust Puro)
Territorio Sagrado. Lógica pura, matemáticas y algoritmos. no_std donde sea posible. Cero dependencias de infraestructura (DB/Red).
Aparato (Lib) Alias (Cargo.toml) Ubicación Descripción
core-math-engine prospector-math libs/core/math-engine Implementación de secp256k1 optimizada con ensamblador inline y SIMD (AVX-512).
core-probabilistic prospector-bloom libs/core/probabilistic Implementación de Filtros de Bloom serializables y ultra-compactos para RAM limitada.
core-generators prospector-gen libs/core/generators Algoritmos de entropía (PRNGs) y derivación de claves (BIP32, WIF, Address Encoding).
ESTRATO 3: DOMAIN (Lógica de Negocio y Estrategia)
El cerebro del sistema. Define "cómo" atacamos y las estructuras de datos compartidas.
Aparato (Lib) Lenguaje Ubicación Descripción
domain-mining-strategy Rust libs/domain/mining-strategy Diccionarios de ataque (Brainwallets), patrones humanos, permutaciones y lógica de búsqueda.
domain-models-rs Rust libs/domain/models-rs Structs y Enums compartidos (ej: TargetRange, FoundWallet, WorkerHeartbeat).
domain-models-ts TS libs/domain/models-ts Tipos TypeScript generados/sincronizados para el Frontend (Zod Schemas).
domain-forensics Rust libs/domain/forensics Lógica de "Arqueología": detección de patrones legacy (Debian RNG bug, Android bug).
ESTRATO 4: INFRASTRUCTURE (Adaptadores y Puertos)
La capa sucia. Aquí viven las conexiones a bases de datos, APIs externas y serialización. Implementa interfaces definidas en Domain.
Aparato (Lib) Lenguaje Ubicación Descripción
infra-db-turso Rust libs/infra/db-turso Cliente asíncrono para Turso (libSQL). Maneja pools de conexión y sharding logic.
infra-bigquery Rust libs/infra/bigquery Cliente para autenticación y descarga streaming de datos de Google BigQuery.
infra-transport Rust libs/infra/transport Capa de serialización "Zero-Copy" (usando rkyv o bincode) para comunicación Orquestador-Minero.
infra-api-client TS libs/infra/api-client-ts Cliente HTTP (TanStack Query) para que el Frontend consuma la API del Orquestador/Turso.
ESTRATO 5: FEATURES & UI (Capa de Presentación - TypeScript)
Aparatos visuales "Plug & Play". Contienen la lógica de UI conectada al estado.
Aparato (Lib) Alias (paths) Ubicación Descripción
feat-rich-list @prospector/feat-rich libs/features/rich-list Visualización D3.js (Bubble Charts) de la distribución de riqueza.
feat-zombie-scan @prospector/feat-zombie libs/features/zombie-scan UI + Lógica para consultar una dirección y ver su probabilidad de ser "Brainwallet".
feat-network-map @prospector/feat-map libs/features/network-map Mapa WebGL de nodos activos en tiempo real.
ui-design-system @prospector/ui-kit libs/shared/ui-kit Componentes atómicos (Botones, Inputs, Cards) basados en Shadcn/Tailwind.
util-formatter @prospector/util-fmt libs/shared/util-formatter Formateadores de Satoshis a BTC/USD, fechas de bloques, etc.
ANÁLISIS DE JUSTIFICACIÓN (ELITE ENGINEERING)
¿Por qué core separado de domain?
Escalabilidad: core-math-engine es matemática pura. Si mañana quieres atacar Ethereum, reutilizas core-math (la curva es la misma) y solo creas un nuevo domain-mining-strategy-eth.
Performance: core puede compilarse con flags agresivos de optimización (-C target-cpu=native) sin afectar al resto.
¿Por qué models-rs y models-ts?
Interoperabilidad: El Backend (Rust) y el Frontend (TS) necesitan hablar el mismo idioma. Usaremos herramientas para generar tipos TS desde Rust automáticamente, manteniendo la Single Source of Truth.
¿Por qué infra-transport separado?
Evolución: Hoy usamos HTTP/JSON. Mañana, para velocidad extrema, cambiaremos a gRPC o TCP Raw sockets. Al tenerlo aislado en un Aparato, cambiamos el protocolo sin tocar ni una línea del Minero ni del Orquestador.
Granularidad de Features UI:
Lazy Loading: Next.js puede cargar el código de feat-rich-list solo si el usuario entra a esa página, reduciendo el tamaño del bundle inicial.
Esta estructura es el plano de un Rascacielos de Ingeniería. Es sólida, modular y está diseñada para sobrevivir al crecimiento exponencial del proyecto.

---
