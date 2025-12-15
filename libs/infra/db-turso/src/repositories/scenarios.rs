// libs/infra/db-turso/src/repositories/scenarios.rs
// =================================================================
// APARATO: TEST SCENARIO REPOSITORY (ELITE EDITION)
// RESPONSABILIDAD: PERSISTENCIA ACID DE ESCENARIOS FORENSES
// MEJORA: IMPLEMENTACIÓN DE 'RETURNING *' PARA ATOMICIDAD
// =================================================================

use crate::errors::DbError;
use crate::TursoClient;
use libsql::params;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Entidad que representa un "Golden Ticket" o escenario de prueba.
///
/// Este struct es público y serializable para permitir su tránsito
/// desde la capa de persistencia hasta la API REST.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestScenario {
    /// Identificador único universal (UUID v4).
    pub id: String,

    /// Nombre descriptivo para el operador (ej: "Debian Bug Simulation").
    pub name: String,

    /// La frase semilla o entropía original (Input secreto).
    pub secret_phrase: String,

    /// La dirección Bitcoin P2PKH esperada (Output público).
    pub target_address: String,

    /// La clave privada WIF esperada (para validación cruzada).
    pub target_private_key: String,

    /// Estado del escenario: 'idle', 'active', 'verified'.
    pub status: String,

    /// Timestamp de creación (ISO 8601 generado por DB).
    pub created_at: String,
}

/// Repositorio especializado en la gestión del Laboratorio Criptográfico.
pub struct ScenarioRepository {
    client: TursoClient,
}

impl ScenarioRepository {
    /// Constructor con inyección de cliente de base de datos.
    pub fn new(client: TursoClient) -> Self {
        Self { client }
    }

    /// Crea un nuevo escenario de prueba de manera atómica.
    ///
    /// # Mejoras de Élite
    /// Utiliza la cláusula `RETURNING *` de SQLite/LibSQL para insertar y recuperar
    /// los datos (incluyendo el `created_at` generado por el servidor) en una sola
    /// operación de red, garantizando consistencia y rendimiento.
    pub async fn create(
        &self,
        name: &str,
        phrase: &str,
        address: &str,
        pk: &str,
    ) -> Result<TestScenario, DbError> {
        let conn = self.client.get_connection()?;
        let id = Uuid::new_v4().to_string();

        // Query optimizada con retorno inmediato
        let query = r#"
            INSERT INTO test_scenarios
            (id, name, secret_phrase, target_address, target_private_key, status)
            VALUES (?1, ?2, ?3, ?4, ?5, 'idle')
            RETURNING id, name, secret_phrase, target_address, target_private_key, status, created_at
        "#;

        let mut rows = conn
            .query(query, params![id, name, phrase, address, pk])
            .await
            .map_err(DbError::QueryError)?;

        // Mapeo inmediato del resultado (Expectativa: 1 fila exacta)
        if let Some(row) = rows.next().await.map_err(DbError::QueryError)? {
            Ok(TestScenario {
                id: row.get(0)?,
                name: row.get(1)?,
                secret_phrase: row.get(2)?,
                target_address: row.get(3)?,
                target_private_key: row.get(4)?,
                status: row.get(5)?,
                created_at: row.get(6)?,
            })
        } else {
            Err(DbError::MappingError(
                "La base de datos no devolvió el registro creado.".to_string(),
            ))
        }
    }

    /// Lista todos los escenarios registrados, ordenados por fecha de creación descendente.
    pub async fn list_all(&self) -> Result<Vec<TestScenario>, DbError> {
        let conn = self.client.get_connection()?;

        let query = r#"
            SELECT id, name, secret_phrase, target_address, target_private_key, status, created_at
            FROM test_scenarios
            ORDER BY created_at DESC
        "#;

        let mut rows = conn.query(query, ()).await.map_err(DbError::QueryError)?;
        let mut scenarios = Vec::new();

        while let Some(row) = rows.next().await.map_err(DbError::QueryError)? {
            scenarios.push(TestScenario {
                id: row.get(0)?,
                name: row.get(1)?,
                secret_phrase: row.get(2)?,
                target_address: row.get(3)?,
                target_private_key: row.get(4)?,
                status: row.get(5)?,
                created_at: row.get(6)?,
            });
        }

        Ok(scenarios)
    }

    /// Busca un escenario específico por su dirección objetivo (The Interceptor Logic).
    ///
    /// Utilizado para validar si una dirección ingresada manualmente corresponde
    /// a un "Golden Ticket" conocido en el sistema.
    pub async fn find_by_address(&self, address: &str) -> Result<Option<TestScenario>, DbError> {
        let conn = self.client.get_connection()?;

        let query = r#"
            SELECT id, name, secret_phrase, target_address, target_private_key, status, created_at
            FROM test_scenarios
            WHERE target_address = ?1
            LIMIT 1
        "#;

        let mut rows = conn
            .query(query, params![address])
            .await
            .map_err(DbError::QueryError)?;

        if let Some(row) = rows.next().await.map_err(DbError::QueryError)? {
            Ok(Some(TestScenario {
                id: row.get(0)?,
                name: row.get(1)?,
                secret_phrase: row.get(2)?,
                target_address: row.get(3)?,
                target_private_key: row.get(4)?,
                status: row.get(5)?,
                created_at: row.get(6)?,
            }))
        } else {
            Ok(None)
        }
    }
}
