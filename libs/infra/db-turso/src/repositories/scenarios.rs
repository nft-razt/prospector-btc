// libs/infra/db-turso/src/repositories/scenarios.rs
// =================================================================
// APARATO: TEST SCENARIO REPOSITORY (GOLD MASTER)
// RESPONSABILIDAD: PERSISTENCIA DE EXPERIMENTOS CRIPTOGRÁFICOS
// ESTADO: FULL IMPLEMENTATION // NO ABBREVIATIONS
// =================================================================

use crate::errors::DbError;
use crate::TursoClient;
use libsql::{params, Row};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Representación atómica de un Escenario de Prueba en la Base de Datos.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestScenario {
    /// Identificador único universal.
    pub id: String,
    /// Nombre designado para la operación.
    pub name: String,
    /// La frase semilla original (Secreto).
    pub secret_phrase: String,
    /// La dirección derivada esperada (Target).
    pub target_address: String,
    /// La clave privada en formato WIF.
    pub target_private_key: String,
    /// Estado del ciclo de vida: idle, active, verified.
    pub status: String,
    /// Fecha de cristalización.
    pub created_at: String,
    /// Fecha de resolución (opcional).
    pub verified_at: Option<String>,
}

pub struct ScenarioRepository {
    client: TursoClient,
}

impl ScenarioRepository {
    /// Inicializa el repositorio con el cliente de Turso inyectado.
    pub fn new(client: TursoClient) -> Self {
        Self { client }
    }

    /// Registra un nuevo escenario y retorna la entidad persistida de forma atómica.
    pub async fn create_atomic(
        &self,
        name: &str,
        phrase: &str,
        address: &str,
        wif: &str,
    ) -> Result<TestScenario, DbError> {
        let connection = self.client.get_connection()?;
        let internal_id = Uuid::new_v4().to_string();

        let query = r#"
            INSERT INTO test_scenarios
            (id, name, secret_phrase, target_address, target_private_key, status)
            VALUES (?1, ?2, ?3, ?4, ?5, 'idle')
            RETURNING id, name, secret_phrase, target_address, target_private_key, status, created_at, verified_at
        "#;

        let mut rows = connection
            .query(query, params![internal_id, name, phrase, address, wif])
            .await
            .map_err(DbError::QueryError)?;

        if let Some(row) = rows.next().await.map_err(DbError::QueryError)? {
            self.map_row_to_entity(row)
        } else {
            Err(DbError::MappingError(
                "Atomic insert failed: No data returned from DB".into(),
            ))
        }
    }

    /// Busca un escenario por su dirección objetivo (Usado por The Interceptor).
    pub async fn find_by_address(&self, address: &str) -> Result<Option<TestScenario>, DbError> {
        let connection = self.client.get_connection()?;

        let query = "SELECT * FROM test_scenarios WHERE target_address = ?1 LIMIT 1";
        let mut rows = connection
            .query(query, params![address.trim()])
            .await
            .map_err(DbError::QueryError)?;

        if let Some(row) = rows.next().await.map_err(DbError::QueryError)? {
            Ok(Some(self.map_row_to_entity(row)?))
        } else {
            Ok(None)
        }
    }

    /// Mapeo estricto de fila SQL a Entidad de Rust.
    fn map_row_to_entity(&self, row: Row) -> Result<TestScenario, DbError> {
        Ok(TestScenario {
            id: row.get(0).map_err(DbError::QueryError)?,
            name: row.get(1).map_err(DbError::QueryError)?,
            secret_phrase: row.get(2).map_err(DbError::QueryError)?,
            target_address: row.get(3).map_err(DbError::QueryError)?,
            target_private_key: row.get(4).map_err(DbError::QueryError)?,
            status: row.get(5).map_err(DbError::QueryError)?,
            created_at: row.get(6).map_err(DbError::QueryError)?,
            verified_at: row.get(7).ok(),
        })
    }
}
