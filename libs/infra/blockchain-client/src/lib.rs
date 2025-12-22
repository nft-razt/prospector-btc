/**
 * =================================================================
 * APARATO: BITCOIN NETWORK UPLINK CLIENT (V10.0 - SOBERANO)
 * CLASIFICACIÓN: INFRASTRUCTURE ADAPTER (ESTRATO L4)
 * RESPONSABILIDAD: EXTRACCIÓN DE VERDAD BLOCHCHAIN EN TIEMPO REAL
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa el túnel de comunicación hacia la red Bitcoin utilizando
 * el API de Blockchain.info. Permite auditar saldos y actividad
 * histórica sin la sobrecarga de un nodo local.
 * =================================================================
 */

use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::time::Duration;
use thiserror::Error;

/// Catálogo de fallos en el enlace con la red Bitcoin.
#[derive(Error, Debug)]
pub enum BlockchainNetworkError {
    #[error("NETWORK_UNREACHABLE: Failed to connect to provider: {0}")]
    ConnectionFault(#[from] reqwest::Error),
    #[error("API_LIMIT_OR_FAULT: Provider rejected the request")]
    ProviderRejection,
}

/// Representación técnica de la actividad de una dirección en el Ledger.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitcoinAddressNetworkState {
    pub final_balance_satoshis: u64,
    pub total_received_satoshis: u64,
    pub confirmed_transaction_count: u32,
}

pub struct BitcoinNetworkUplinkClient {
    internal_http_session: Client,
}

impl BitcoinNetworkUplinkClient {
    /**
     * Construye una instancia con timeouts de grado industrial.
     */
    pub fn new() -> Self {
        Self {
            internal_http_session: Client::builder()
                .timeout(Duration::from_secs(15))
                .user_agent("Prospector-Audit-Engine/V10.8")
                .build()
                .expect("CRITICAL: Failed to initialize HTTP session"),
        }
    }

    /**
     * Recupera la verdad histórica y actual de una dirección Bitcoin.
     *
     * @param target_bitcoin_address Dirección en formato Base58Check.
     */
    pub async fn fetch_bitcoin_address_activity(
        &self,
        target_bitcoin_address: &str
    ) -> Result<BitcoinAddressNetworkState, BlockchainNetworkError> {
        let source_url = format!("https://blockchain.info/rawaddr/{}", target_bitcoin_address);

        let network_response = self.internal_http_session
            .get(&source_url)
            .send()
            .await?;

        if !network_response.status().is_success() {
            return Err(BlockchainNetworkError::ProviderRejection);
        }

        #[derive(Deserialize)]
        struct BlockchainInfoSchema {
            final_balance: u64,
            total_received: u64,
            n_tx: u32,
        }

        let decoded_payload: BlockchainInfoSchema = network_response.json().await?;

        Ok(BitcoinAddressNetworkState {
            final_balance_satoshis: decoded_payload.final_balance,
            total_received_satoshis: decoded_payload.total_received,
            confirmed_transaction_count: decoded_payload.n_tx,
        })
    }
}

impl Default for BitcoinNetworkUplinkClient {
    fn default() -> Self {
        Self::new()
    }
}
