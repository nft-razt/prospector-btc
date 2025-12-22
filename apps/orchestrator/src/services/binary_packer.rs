/**
 * =================================================================
 * APARATO: BINARY NEURAL PACKER (V65.0 - SOBERANO)
 * CLASIFICACIÓN: INFRASTRUCTURE SERVICE (ESTRATO L4)
 * RESPONSABILIDAD: COMPRESIÓN DE SEÑALES SSE MEDIANTE MESSAGEPACK
 * =================================================================
 */

use prospector_domain_models::telemetry::RealTimeEvent;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use rmp_serde::to_vec_named;
use tracing::error;

pub struct BinaryNeuralPacker;

impl BinaryNeuralPacker {
    /**
     * Empaqueta un evento de dominio en una cadena Base64-MessagePack.
     *
     * @param event El evento de tiempo real a transmitir.
     * @returns Option con el string serializado listo para SSE.
     */
    pub fn pack_event(event: &RealTimeEvent) -> Option<String> {
        // Serialización eficiente a binario compacto.
        let binary_payload = match to_vec_named(event) {
            Ok(bytes) => bytes,
            Err(serialization_error) => {
                error!("❌ [PACKER_FAULT]: MessagePack failure: {}", serialization_error);
                return None;
            }
        };

        // Codificación Base64 para transporte seguro sobre túnel SSE (Textual).
        Some(BASE64.encode(binary_payload))
    }
}
