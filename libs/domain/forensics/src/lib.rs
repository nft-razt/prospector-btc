// libs/domain/forensics/src/lib.rs
// =================================================================
// APARATO: FORENSICS MODULE BARREL
// =================================================================

pub mod android_rng;
pub mod debian_rng; // ✅ NUEVO MÓDULO

pub use android_rng::AndroidLcgIterator;
pub use debian_rng::DebianIterator; // ✅ EXPORTACIÓN
