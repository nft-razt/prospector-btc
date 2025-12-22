/**
 * =================================================================
 * APARATO: FORENSIC VECTOR AUDITOR (V10.3 - DOCTORAL STANDARDS)
 * CLASIFICACIÓN: DOMAIN STRATEGY (ESTRATO L2)
 * RESPONSABILIDAD: AUDITORÍA INTEGRAL DEL DATASET DE PRUEBA
 * =================================================================
 */

use prospector_core_math::prelude::*;
use prospector_core_gen::address_legacy::pubkey_to_address;
use prospector_core_gen::wif::private_to_wif;
use crate::phrase_to_private_key;
use prospector_blockchain_client::{BitcoinNetworkUplinkClient, BitcoinAddressNetworkState};
use serde::{Serialize, Deserialize};
use futures::future::join_all;

/// Reporte consolidado de un vector del dataset de prueba.
///
/// Contiene la validación simétrica de la matemática y la verdad
/// extraída de la red Bitcoin.
#[derive(Debug, Serialize, Deserialize)]
pub struct VerifiedVectorAuditReport {
    /// Identificador único del vector dentro del manifiesto soberano.
    pub vector_identifier: u32,
    /// Frase de entropía original utilizada para iniciar la derivación.
    pub source_passphrase: String,
    /// Clave privada en formato WIF comprimido generada por el motor.
    pub derived_wallet_import_format: String,
    /// Dirección Bitcoin Legacy (P2PKH) resultante de la derivación.
    pub derived_bitcoin_address: String,
    /// Booleano que certifica que la matemática local coincide con el dataset de referencia.
    pub mathematical_integrity_verified: bool,
    /// Información técnica y saldo actual recuperado de la Blockchain real.
    pub network_reality_data: Option<BitcoinAddressNetworkState>,
}

/// Motor de auditoría para la certificación de vectores de control conocidos.
pub struct ForensicVectorAuditor;

impl ForensicVectorAuditor {
    /**
     * Ejecuta una secuencia de certificación paralela sobre una colección de vectores.
     *
     * # Performance
     * Utiliza un modelo de ejecución asíncrono basado en `futures::future::join_all`
     * para minimizar el tiempo total de espera de las peticiones de red.
     *
     * @param input_vectors Colección de tuplas conteniendo los datos de referencia.
     * @returns Vector de reportes de auditoría certificados.
     */
    pub async fn execute_dataset_certification(
        input_vectors: Vec<(u32, String, String, String, String)>
    ) -> Vec<VerifiedVectorAuditReport> {
        let network_uplink_client = BitcoinNetworkUplinkClient::new();

        let audit_tasks = input_vectors.into_iter().map(|(id, _type, phrase, expected_wif, expected_address)| {
            let client_reference = &network_uplink_client;
            async move {
                // 1. DERIVACIÓN CRIPTOGRÁFICA (L1/L2)
                let private_key_instance = phrase_to_private_key(&phrase);
                let public_key_instance = SafePublicKey::from_private(&private_key_instance);

                let actual_wif_compressed = private_to_wif(&private_key_instance, true);
                let actual_bitcoin_address = pubkey_to_address(&public_key_instance, true);

                // 2. VERIFICACIÓN DE SIMETRÍA
                let is_mathematically_correct =
                    actual_bitcoin_address == expected_address &&
                    actual_wif_compressed == expected_wif;

                // 3. CONSULTA DE VERDAD EN LA RED (Uplink L4)
                let live_network_state = client_reference
                    .fetch_bitcoin_address_activity(&actual_bitcoin_address)
                    .await
                    .ok();

                VerifiedVectorAuditReport {
                    vector_identifier: id,
                    source_passphrase: phrase,
                    derived_wallet_import_format: actual_wif_compressed,
                    derived_bitcoin_address: actual_bitcoin_address,
                    mathematical_integrity_verified: is_mathematically_correct,
                    network_reality_data: live_network_state,
                }
            }
        });

        join_all(audit_tasks).await
    }
}
