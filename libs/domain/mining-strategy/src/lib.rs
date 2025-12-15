pub mod brainwallet;
pub mod combinatoric;
pub mod dictionary;
pub mod executor;

// Módulo de pruebas
mod tests_execution;

// Re-exports para consumo fácil
pub use brainwallet::BrainwalletIterator;
pub use combinatoric::CombinatoricIterator;
pub use dictionary::DictionaryIterator;
pub use executor::{ExecutorContext, FindingHandler, StrategyExecutor};
