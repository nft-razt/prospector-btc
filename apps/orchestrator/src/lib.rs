/**
 * =================================================================
 * APARATO: ORCHESTRATOR LOGIC HUB (V1.2 - SOBERANO)
 * =================================================================
 */

pub mod bootstrap;
pub mod bootstrap_forensics;
pub mod handlers;
pub mod kernel;
pub mod middleware;
pub mod routes;
pub mod services;
pub mod state;

pub mod prelude {
    pub use crate::kernel::OrchestratorKernel;
    pub use crate::state::AppState; // ✅ CORRECCIÓN: Eliminado .mod::
}
