// libs/domain/models-rs/src/work.rs
use serde::{Serialize, Deserialize};
use typeshare::typeshare; // <--- IMPORTANTE

/// Define una unidad de trabajo.
/// El decorador #[typeshare] generará automáticamente la interfaz TS equivalente.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkOrder {
    pub id: String,
    pub strategy: SearchStrategy,
    pub target_duration_sec: u64,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "params")]
pub enum SearchStrategy {
    Combinatoric {
        prefix: String,
        suffix: String,
        start_index: String, // String para BigInt en JS
        end_index: String,
    },
    Dictionary {
        dataset_url: String,
        limit: usize,
    },
    Random {
        seed: u64, // Ojo: JS SafeInteger es 2^53
    },
}
