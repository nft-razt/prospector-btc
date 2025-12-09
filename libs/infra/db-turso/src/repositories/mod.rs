pub mod finding;
// Rust buscará ahora dentro de la carpeta 'job/mod.rs' automáticamente
pub mod job;
pub mod identity;

pub use finding::FindingRepository;
pub use job::JobRepository;
pub use identity::IdentityRepository;
