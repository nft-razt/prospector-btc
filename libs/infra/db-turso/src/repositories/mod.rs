// libs/infra/db-turso/src/repositories/mod.rs
// =================================================================
// APARATO: REPOSITORY ACCESS MATRIX (V14.0)
// RESPONSABILIDAD: GESTIÓN DE VISIBILIDAD DE LOS ADAPTADORES DE DATOS
// ESTADO: RESOLUCIÓN DE ERROR rustc(unresolved import)
// =================================================================

/// Módulo de persistencia para hallazgos criptográficos (Collisions).
pub mod finding;

/// Módulo de gestión del Ledger de trabajos (Ranges).
pub mod job;

/// Módulo de gestión de identidades y credenciales (IAM).
pub mod identity;

/// Módulo de telemetría y salud de nodos (Fleet).
pub mod worker;

/// Módulo de gestión de experimentos y Golden Tickets (QA).
pub mod scenarios;

/// Módulo de servicios de archivo estratégico (Migration).
pub mod archival;

// --- RE-EXPORTS PÚBLICOS (SSoT) ---
// Facilitamos el acceso a los repositorios desde el Orquestador
// manteniendo el principio de ocultación de la implementación.

pub use finding::FindingRepository;
pub use job::JobRepository;
pub use identity::IdentityRepository;
pub use worker::WorkerRepository;
pub use archival::ArchivalRepository;

// ✅ NIVELACIÓN: Exportación limpia de la infraestructura de Laboratorio.
// Aseguramos que tanto el repositorio como la entidad de persistencia sean accesibles.
pub use scenarios::{ScenarioRepository, TestScenario};
