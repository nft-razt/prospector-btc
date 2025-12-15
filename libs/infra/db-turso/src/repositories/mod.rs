// libs/infra/db-turso/src/repositories/mod.rs
// =================================================================
// APARATO: REPOSITORY EXPORTS
// RESPONSABILIDAD: API PÚBLICA DE LA CAPA DE DATOS
// =================================================================

pub mod finding;
pub mod job;
pub mod identity;
pub mod worker;
pub mod scenarios;

// Re-exports planos para facilitar el uso en apps/orchestrator
pub use finding::FindingRepository;
pub use job::JobRepository;
pub use identity::IdentityRepository;
pub use worker::WorkerRepository;

// ✅ EXPORTACIÓN CRÍTICA: Struct de datos + Repositorio
pub use scenarios::{ScenarioRepository, TestScenario};
