pub mod brainwallet;
pub mod combinatoric;
pub mod dictionary; // <--- NUEVO
pub mod executor;   // <--- ACTUALIZADO

// Re-exports para consumo fácil
pub use brainwallet::BrainwalletIterator;
pub use combinatoric::CombinatoricIterator;
pub use dictionary::DictionaryIterator;
pub use executor::{StrategyExecutor, ExecutorContext, FindingHandler};
