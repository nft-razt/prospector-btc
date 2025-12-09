
use serde::{Serialize, Deserialize};

/// El "Santo Grial". Estructura que se reporta cuando hay una colisión.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    /// La dirección Bitcoin (P2PKH) colisionada.
    pub address: String,

    /// La clave privada en formato WIF (Wallet Import Format).
    /// ¡DATOS SENSIBLES!
    pub private_key_wif: String,

    /// Qué generó esta clave (ej: "brainwallet:password123").
    pub source_entropy: String,

    /// Tipo de billetera (Legacy, Segwit, etc).
    pub wallet_type: String,
}
