/**
 * =================================================================
 * APARATO: NEURAL STREAM HANDLER (V155.0 - BINARY TUNNEL)
 * RESPONSABILIDAD: EMISIÓN DE EVENTOS SSE EMPAQUETADOS
 * =================================================================
 */

use crate::state::AppState;
use crate::services::binary_packer::BinaryNeuralPacker;
use axum::{
    extract::State,
    response::sse::{Event, KeepAlive, Sse},
};
use futures::stream::Stream;
use std::convert::Infallible;
use std::time::Duration;
use tokio_stream::StreamExt;
use tokio_stream::wrappers::BroadcastStream;

pub async fn stream_metrics(
    State(application_state): State<AppState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {

    // Suscripción al bus de eventos nivelado
    let event_receiver = application_state.event_bus.subscribe();

    let event_stream = BroadcastStream::new(event_receiver).map(|result| {
        match result {
            Ok(domain_event) => {
                // EMPAQUETAMIENTO DE ÉLITE
                if let Some(packed_payload) = BinaryNeuralPacker::pack_event(&domain_event) {
                    Ok(Event::default().data(packed_payload))
                } else {
                    Ok(Event::default().comment("serialization_error"))
                }
            },
            Err(_) => Ok(Event::default().comment("stream_lagged")),
        }
    });

    Sse::new(event_stream).keep_alive(
        KeepAlive::new()
            .interval(Duration::from_secs(15))
            .text("heartbeat")
    )
}
