// libs/core/client-vault-rs/src/lib.rs
/**
 * =================================================================
 * APARATO: CRYPTOGRAPHIC VAULT ENGINE (RUST EDITION V13.0)
 * CLASIFICACIÓN: CORE SECURITY (L1)
 * RESPONSABILIDAD: DESCIFRADO SIMÉTRICO ZERO-KNOWLEDGE (AES-GCM)
 *
 * ESTRATEGIA DE ÉLITE:
 * - Algoritmo: AES-256-GCM (Autenticación y Cifrado).
 * - Derivación: PBKDF2-HMAC-SHA256 // 100,000 Iteraciones.
 * - Sincronía: Compatible 1:1 con WebCrypto API (L1-TS).
 * =================================================================
 */

use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce, Key
};
use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VaultError {
    #[error("Base64 decoding failed: {0}")]
    EncodingError(#[from] base64::DecodeError),
    #[error("Key derivation failed")]
    DerivationError,
    #[error("Decryption malfunction: Data integrity compromised or wrong key")]
    DecryptionError,
}

/// Payload cifrado recibido desde la Bóveda Estratégica.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EncryptedVaultPayload {
    pub cipher_text_base64: String,
    pub initialization_vector_base64: String,
    pub salt_base64: String,
}

pub struct VaultCryptoEngine;

impl VaultCryptoEngine {
    const ITERATIONS: u32 = 100_000;
    const KEY_LEN: usize = 32; // 256 bits

    /**
     * Descifra un payload Zero-Knowledge utilizando la clave maestra del operador.
     *
     * # Parámetros
     * * `payload` - Estructura con Cipher, IV y Salt.
     * * `master_key` - Frase de paso inyectada en el arranque del worker.
     */
    pub fn decrypt(payload: &EncryptedVaultPayload, master_key: &str) -> Result<String, VaultError> {
        // 1. Decodificación de Material Base64
        let cipher_text = BASE64.decode(&payload.cipher_text_base64)?;
        let iv = BASE64.decode(&payload.initialization_vector_base64)?;
        let salt = BASE64.decode(&payload.salt_base64)?;

        // 2. Derivación de la Clave (Sincronizada con L1-TS)
        let mut derived_key = [0u8; Self::KEY_LEN];
        pbkdf2_hmac::<Sha256>(
            master_key.as_bytes(),
            &salt,
            Self::ITERATIONS,
            &mut derived_key
        );

        // 3. Inicialización del Motor AES-GCM
        let key = Key::<Aes256Gcm>::from_slice(&derived_key);
        let cipher = Aes256Gcm::new(key);
        let nonce = Nonce::from_slice(&iv);

        // 4. Ejecución del Descifrado Atómico
        let decrypted_bytes = cipher
            .decrypt(nonce, cipher_text.as_ref())
            .map_err(|_| VaultError::DecryptionError)?;

        String::from_utf8(decrypted_bytes)
            .map_err(|_| VaultError::DecryptionError)
    }
}
