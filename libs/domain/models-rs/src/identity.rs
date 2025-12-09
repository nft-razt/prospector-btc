// =================================================================
// APARATO: IDENTITY MODELS
// RESPONSABILIDAD: DEFINICIÓN DE TIPOS DE DATOS (DTOs)
// =================================================================

use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Representa una identidad digital completa y auditable.
/// Mapea directamente a la tabla 'identities' en Turso.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identity {
    pub id: Uuid,

    /// Plataforma objetivo (ej: "google_colab", "ideogram")
    pub platform: String,

    /// Identificador principal (Email)
    pub email: String,

    /// Cookies o Tokens en JSON string (almacenado como texto en DB)
    pub credentials_json: String,

    /// User-Agent asociado para evasión de fingerprinting
    pub user_agent: String,

    /// Estadísticas de uso para rotación inteligente
    pub usage_count: u64,
    pub last_used_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,

    pub status: IdentityStatus,
}

/// Estado del ciclo de vida de una identidad.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum IdentityStatus {
    Active,
    RateLimited, // 429 Detectado: Pausar temporalmente
    Expired,     // Cookies caducadas: Requiere renovación manual
    Revoked,     // Credenciales inválidas: Descartar
}

/// DTO para la creación/subida de una identidad desde el Dashboard.
#[derive(Debug, Deserialize)]
pub struct CreateIdentityPayload {
    pub platform: String,
    pub email: String,

    /// Recibe un objeto JSON arbitrario (cookies array).
    /// El uso de 'serde_json::Value' permite flexibilidad total en el formato de cookies.
    pub cookies: serde_json::Value,

    pub user_agent: String,
}
