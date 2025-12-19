/**
 * =================================================================
 * APARATO: REPOSITORY ACCESS MATRIX (V18.0)
 * CLASIFICACIÓN: INFRASTRUCTURE MAPPING (L3)
 * RESPONSABILIDAD: ORQUESTACIÓN DE VISIBILIDAD DE SUBSISTEMAS
 * =================================================================
 */

pub mod finding;
pub mod identity;
pub mod worker;
pub mod scenarios;
pub mod archival;
pub mod audit_repository;
pub mod mission_repository; // ✅ NIVELACIÓN: Integración del nuevo secuenciador

// --- EXPORTACIONES SOBERANAS (Zero Regressions) ---

pub use archival::ArchivalRepository;
pub use finding::FindingRepository;
pub use identity::IdentityRepository;
pub use mission_repository::MissionRepository; // ✅ REEMPLAZO: Superior a JobRepository
pub use worker::WorkerRepository;
pub use scenarios::{ScenarioRepository, TestScenario};
pub use audit_repository::AuditRepository;
