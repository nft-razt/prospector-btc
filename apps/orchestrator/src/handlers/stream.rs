// apps/orchestrator/src/handlers/stream.rs
// =================================================================
// APARATO: SSE STREAM HANDLER (v6.8 - POLISHED)
// RESPONSABILIDAD: GATEWAY DE PUSH NOTIFICATIONS (SERVER-SENT EVENTS)
// CORRECCIONES:
// - Ruta de importación de BroadcastStreamRecvError arreglada.
// - Limpieza de imports no utilizados (debug, JsonError).
// =================================================================

use axum::{
    extract::State,
    response::sse::{Event, KeepAlive, Sse},
};
use tokio_stream::wrappers::BroadcastStream;
// ✅ CORRECCIÓN CRÍTICA: Importación desde el submódulo 'errors'
use futures::stream::Stream;
use std::convert::Infallible;
use std::time::Duration;
use tokio_stream::wrappers::errors::BroadcastStreamRecvError;
use tokio_stream::StreamExt;
use tracing::{info, warn}; // ✅ CORRECCIÓN: 'debug' eliminado

use crate::state::AppState;
use prospector_domain_models::RealTimeEvent;

/// Constantes de Configuración SSE
const KEEPALIVE_INTERVAL_SEC: u64 = 15;

/// Transforma un evento de dominio en un evento SSE de transporte.
///
/// Esta función actúa como un adaptador de tipos estricto.
/// Entrada: Result<RealTimeEvent, BroadcastStreamRecvError>
/// Salida:  Result<Event, Infallible> (Requerido por Axum Sse)
fn map_domain_event_to_sse(
    event_result: Result<RealTimeEvent, BroadcastStreamRecvError>,
) -> Result<Event, Infallible> {
    match event_result {
        // CASO 1: Evento recibido correctamente del Bus
        Ok(event) => {
            match serde_json::to_string(&event) {
                Ok(json) => {
                    // Empaquetamos el JSON en un evento SSE 'data'
                    Ok(Event::default().data(json))
                }
                Err(e) => {
                    // Fallo Lógico: El evento no es serializable.
                    // Reportamos como comentario para no romper el stream.
                    warn!("⚠️ SSE Serialization Failure: {}", e);
                    Ok(Event::default().comment(format!("serialization_error: {}", e)))
                }
            }
        }
        // CASO 2: Lag en el canal (El consumidor es demasiado lento)
        Err(_) => {
            // BroadcastStreamRecvError::Lagged
            Ok(Event::default().comment("lagged_connection_frames_dropped"))
        }
    }
}

/// Endpoint: GET /api/v1/stream/metrics
///
/// Establece una conexión persistente unidireccional.
/// Utiliza `BroadcastStream` para suscribirse al bus de eventos global.
pub async fn stream_metrics(
    State(state): State<AppState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    // 1. Suscripción Atómica
    let rx = state.events.subscribe();
    let stream = BroadcastStream::new(rx);

    info!("🔌 NEURAL LINK: Cliente SSE conectado y suscrito al bus.");

    // 2. Transformación de Stream (Mapping)
    // Aplicamos la función transformadora
    let event_stream = stream.map(map_domain_event_to_sse);

    // 3. Configuración de Transporte (Keep-Alive)
    Sse::new(event_stream).keep_alive(
        KeepAlive::new()
            .interval(Duration::from_secs(KEEPALIVE_INTERVAL_SEC))
            .text("keep-alive-ping"),
    )
}
