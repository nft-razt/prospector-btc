/**
 * =================================================================
 * APARATO: SERVICE ORCHESTRATION HUB (V20.0 - SOBERANO)
 * CLASIFICACIÓN: APPLICATION SERVICES (ESTRATO L4)
 * RESPONSABILIDAD: EXPOSICIÓN UNIFICADA DE DAEMONS Y COORDINADORES
 *
 * VISION HIPER-HOLÍSTICA:
 * Actúa como la matriz de visibilidad para los servicios de fondo.
 * Define alias semánticos para garantizar que el Kernel pueda orquestar
 * la ignición del sistema sin dependencias de nombres de archivos.
 * =================================================================
 */

pub mod binary_packer;
pub mod c2_coordinator;
pub mod certification_authority;
pub mod chronos;
pub mod chronos_archive;
pub mod event_bus;
pub mod finding_flusher;
pub mod flush;
pub mod mission_hydrator;
pub mod outbox_relay;
pub mod parity_auditor;
pub mod reaper;
pub mod swarm_resurrection;
pub mod telemetry;

// --- RE-EXPORTACIONES SOBERANAS ---

/// Alias para el puente de archivo estratégico (Motor B).
pub use outbox_relay::SovereignArchivalEngine as OutboxRelayService;

/// Punto de entrada para el daemon de preservación de instancia.
pub use chronos::spawn_chronos;

/// Daemon de persistencia diferida de telemetría.
pub use flush::spawn_flush_service;

/// Daemon de limpieza de memoria volátil.
pub use reaper::spawn_reaper;

/// Motor de métricas de alta frecuencia.
pub use telemetry::spawn_telemetry_loop;
