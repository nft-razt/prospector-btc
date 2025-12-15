// libs/domain/mining-strategy/src/lib.rs

pub mod brainwallet;
pub mod combinatoric;
pub mod dictionary;
pub mod kangaroo; // ✅ NUEVO MÓDULO
pub mod executor;

// Módulo de pruebas
mod tests_execution;

// Re-exports
pub use brainwallet::BrainwalletIterator;
pub use combinatoric::CombinatoricIterator;
pub use dictionary::DictionaryIterator;
pub use kangaroo::KangarooRunner; // ✅ EXPORT
pub use executor::{ExecutorContext, FindingHandler, StrategyExecutor};
