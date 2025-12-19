// libs/domain/models-rs/src/identity.rs
/**
 * =================================================================
 * APARATO: IDENTITY DOMAIN MODELS (V11.0 - ZK READY)
 * CLASIFICACIÓN: DOMAIN ENTITIES (L2)
 * RESPONSABILIDAD: DEFINICIÓN DE IDENTIDADES Y PAYLOADS CIFRADOS
 * ESTADO: GOLD MASTER // NO ABBREVIATIONS
 * =================================================================
 */
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Estructura del Payload Cifrado en el Cliente (AES-256-GCM).
/// Sincronizado con VaultCryptoEngine (L1 - TypeScript).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedIdentityPayload {
    /// Texto cifrado codificado en Base64.
    pub cipher_text_base64: String,
    /// Vector de inicialización único para esta operación.
    pub initialization_vector_base64: String,
    /// Sal de derivación utilizada para la llave maestra.
    pub salt_base64: String,
}

/// Entidad de Identidad Soberana para el acceso a Runtimes Cloud.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identity {
    pub id: Uuid,
    /// Plataforma de ejecución (ej: "google_colab", "kaggle").
    pub platform: String,
    /// Identificador del operador (Email).
    pub email: String,
    /// Almacenamiento flexible de credenciales (Cifradas o Crudas).
    pub credentials_json: String,
    /// Huella digital del navegador del operador.
    pub user_agent: String,
    pub usage_count: u64,
    pub last_used_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub status: IdentityStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum IdentityStatus {
    Active,
    RateLimited,
    Expired,
    Revoked,
}

/// DTO para la ingesta de identidades desde el Dashboard.
#[derive(Debug, Deserialize)]
pub struct CreateIdentityPayload {
    pub platform: String,
    pub email: String,
    /// Soporta tanto el nuevo EncryptedIdentityPayload como el formato legacy.
    pub cookies: serde_json::Value,
    pub user_agent: String,
}

#[derive(Debug, Deserialize)]
pub struct RevokeIdentityPayload {
    pub email: String,
}
