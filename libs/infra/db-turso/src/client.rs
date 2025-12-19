// libs/infra/db-turso/src/client.rs
// =================================================================
// APARATO: DATABASE CONNECTION CLIENT (V16.0)
// CLASIFICACIÓN: INFRASTRUCTURE LAYER (L3)
// RESPONSABILIDAD: GESTIÓN DE ENLACES TÁCTICOS Y POOLING DE CONEXIONES
//
// CARACTERÍSTICAS DE ÉLITE:
// - Inicialización Idempotente: Sincroniza el esquema completo al conectar.
// - Soporte Híbrido: Conectividad transparente para Local (SQLite) y Nube (libSQL).
// - Gestión de Memoria: Uso de Arc (Atomic Reference Counting) para compartir el Driver.
// =================================================================

use crate::errors::DbError;
use crate::schema::apply_full_schema; // ✅ RESOLUCIÓN: Referencia actualizada al motor V13.0
use libsql::{Builder, Connection, Database};
use std::sync::Arc;
use tracing::{error, info, instrument};

/// Cliente encapsulado para la gestión de persistencia en el ecosistema Prospector.
///
/// Actúa como el túnel de comunicación entre el Orquestador y el Ledger Táctico.
#[derive(Clone)]
pub struct TursoClient {
    /// Instancia interna del Driver de Base de Datos protegida para uso multihilo.
    internal_database_driver: Arc<Database>,
}

impl TursoClient {
    /// Establece un nuevo enlace con la base de datos y sincroniza su estructura estructural.
    ///
    /// # Argumentos
    /// * `database_url`: Localizador de recursos (ej: "file:prospector.db" o "libsql://...").
    /// * `auth_token`: Credencial de seguridad (requerido para nodos en la nube).
    ///
    /// # Flujo de Ignición
    /// 1. Construye el Driver basado en el protocolo de la URL.
    /// 2. Establece una conexión inicial para validación estructural.
    /// 3. Ejecuta la suite de sincronización de esquemas.
    #[instrument(skip(auth_token))]
    pub async fn connect(database_url: &str, auth_token: Option<String>) -> Result<Self, DbError> {
        info!(
            "🔌 DATABASE: Initiating tactical link to [{}]",
            database_url
        );

        // 1. Construcción del Driver de Base de Datos
        let database_driver = if let Some(token) = auth_token {
            Builder::new_remote(database_url.to_string(), token)
                .build()
                .await
                .map_err(|error| {
                    DbError::ConnectionError(format!("Remote ignition failed: {}", error))
                })?
        } else {
            Builder::new_local(database_url)
                .build()
                .await
                .map_err(|error| {
                    DbError::ConnectionError(format!("Local ignition failed: {}", error))
                })?
        };

        // 2. Validación y Sincronización Estructural
        // Creamos una conexión temporal para aplicar el esquema de forma segura.
        let bootstrap_connection = database_driver.connect().map_err(|error| {
            DbError::ConnectionError(format!("Bootstrap link failure: {}", error))
        })?;

        // Invocamos al motor de esquemas nivelado
        apply_full_schema(&bootstrap_connection)
            .await
            .map_err(|error| {
                error!("❌ DATABASE_SCHEMA_ERROR: Structural synchronization failed.");
                DbError::ConnectionError(format!("Schema engine failure: {}", error))
            })?;

        info!("✅ DATABASE: Tactical link secured and synchronized.");

        Ok(Self {
            internal_database_driver: Arc::new(database_driver),
        })
    }

    /// Genera una nueva conexión ligera desde el pool interno del Driver.
    ///
    /// Esta operación es de bajo coste y debe ser utilizada en cada transacción atómica.
    pub fn get_connection(&self) -> Result<Connection, DbError> {
        self.internal_database_driver.connect().map_err(|error| {
            DbError::ConnectionError(format!("Connection pool exhaustion: {}", error))
        })
    }
}
