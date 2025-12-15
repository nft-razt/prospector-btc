// libs/infra/db-turso/src/repositories/scenarios.rs
// =================================================================
// APARATO: TEST SCENARIO REPOSITORY
// RESPONSABILIDAD: ACCESO A DATOS DEL LABORATORIO
// =================================================================

use crate::TursoClient;
use crate::errors::DbError;
use serde::{Serialize, Deserialize};
use libsql::params;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct TestScenario {
    pub id: String,
    pub name: String,
    pub secret_phrase: String,
    pub target_address: String,
    pub target_private_key: String,
    pub status: String,
    pub created_at: String,
}

pub struct ScenarioRepository {
    client: TursoClient,
}

impl ScenarioRepository {
    pub fn new(client: TursoClient) -> Self {
        Self { client }
    }

    pub async fn create(&self, name: &str, phrase: &str, address: &str, pk: &str) -> Result<String, DbError> {
        let conn = self.client.get_connection()?;
        let id = Uuid::new_v4().to_string();

        conn.execute(
            "INSERT INTO test_scenarios (id, name, secret_phrase, target_address, target_private_key, status) VALUES (?1, ?2, ?3, ?4, ?5, 'idle')",
            params![id.clone(), name, phrase, address, pk]
        ).await.map_err(DbError::QueryError)?;

        Ok(id)
    }

    pub async fn list_all(&self) -> Result<Vec<TestScenario>, DbError> {
        let conn = self.client.get_connection()?;
        let mut rows = conn.query("SELECT id, name, secret_phrase, target_address, target_private_key, status, created_at FROM test_scenarios ORDER BY created_at DESC", ()).await.map_err(DbError::QueryError)?;

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

    // ✅ MÉTODO CRÍTICO PARA EL INTERCEPTOR
    pub async fn find_by_address(&self, address: &str) -> Result<Option<TestScenario>, DbError> {
        let conn = self.client.get_connection()?;
        let mut rows = conn.query(
            "SELECT id, name, secret_phrase, target_address, target_private_key, status, created_at FROM test_scenarios WHERE target_address = ?1 LIMIT 1",
            params![address]
        ).await.map_err(DbError::QueryError)?;

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
