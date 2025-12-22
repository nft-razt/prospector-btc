/**
 * =================================================================
 * APARATO: FORENSIC VECTOR AUDITOR (V10.0 - TOTAL SYNC)
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
#[derive(Debug, Serialize, Deserialize)]
pub struct VerifiedVectorAuditReport {
    pub vector_id: u32,
    pub source_passphrase: String,
    pub derived_wif_compressed: String,
    pub derived_bitcoin_address: String,
    pub mathematical_integrity_verified: bool,
    pub network_reality_data: Option<BitcoinAddressNetworkState>,
}

pub struct ForensicVectorAuditor;

impl ForensicVectorAuditor {
    /**
     * Ejecuta una auditoría paralela sobre los 33 vectores proporcionados.
     * Utiliza el motor matemático para validar la derivación y el
     * uplink para validar la existencia en la red.
     */
    pub async fn execute_dataset_certification(
        input_vectors: Vec<(u32, String, String, String)>
    ) -> Vec<VerifiedVectorAuditReport> {
        let network_client = BitcoinNetworkUplinkClient::new();

        let audit_tasks = input_vectors.into_iter().map(|(id, _type, phrase, expected_wif, expected_addr)| {
            let client_ref = &network_client;
            async move {
                // 1. DERIVACIÓN SOBERANA (L1/L2)
                let private_key = phrase_to_private_key(&phrase);
                let public_key = SafePublicKey::from_private(&private_key);

                let actual_wif = private_to_wif(&private_key, true);
                let actual_address = pubkey_to_address(&public_key, true);

                // 2. VALIDACIÓN DE INTEGRIDAD DEL DATASET
                let math_integrity = actual_address == expected_addr && actual_wif == expected_wif;

                // 3. CONSULTA DE RED EN TIEMPO REAL
                let network_state = client_ref.fetch_bitcoin_address_activity(&actual_address).await.ok();

                VerifiedVectorAuditReport {
                    vector_id: id,
                    source_passphrase: phrase,
                    derived_wif_compressed: actual_wif,
                    derived_bitcoin_address: actual_address,
                    mathematical_integrity_verified: math_integrity,
                    network_reality_data: network_state,
                }
            }
        });

        join_all(audit_tasks).await
    }
}
