/**
 * =================================================================
 * APARATO: REPOSITORY ACCESS MATRIX (V20.0 - SOBERANO)
 * CLASIFICACIÓN: INFRASTRUCTURE LAYER (ESTRATO L3)
 * RESPONSABILIDAD: EXPOSICIÓN DE SUBSISTEMAS DE PERSISTENCIA
 *
 * VISION HIPER-HOLÍSTICA:
 * Actúa como el hub de visibilidad para todos los repositorios del
 * Ledger Táctico. Garantiza que el Orquestador y el Prover puedan
 * acceder a la lógica de escenarios, misiones y auditoría.
 * =================================================================
 */

pub mod archival;
pub mod audit_repository;
pub mod finding;
pub mod identity;
pub mod mission_repository;
pub mod scenario_assets;
pub mod scenario_repository; // ✅ RESOLUCIÓN: Exposición explícita para bootstrap_forensics
pub mod scenarios;
pub mod system_repository;   // ✅ RESOLUCIÓN: Exposición explícita para el Kernel
pub mod worker;

// --- RE-EXPORTACIONES DE ÉLITE (Para simplificar imports externos) ---
pub use archival::ArchivalRepository;
pub use audit_repository::AuditRepository;
pub use finding::FindingRepository;
pub use identity::IdentityRepository;
pub use mission_repository::MissionRepository;
pub use scenarios::ScenarioRepository;
pub use worker::WorkerRepository;
pub use system_repository::SystemStateRepository;
pub use scenario_repository::ScenarioRegistryRepository;
