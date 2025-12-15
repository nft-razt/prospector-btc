// libs/infra/db-turso/src/repositories/scenarios.rs
// =================================================================
// APARATO: TEST SCENARIO REPOSITORY (ELITE EDITION)
// RESPONSABILIDAD: PERSISTENCIA TRANSACCIONAL DEL LABORATORIO CRIPTOGRÁFICO
// CARACTERÍSTICAS:
// - Atomicidad: Uso de 'RETURNING *' para reducir RTT (Round Trip Time).
// - Integridad: Tipado fuerte y validación de esquemas.
// - Ciclo Cerrado: Capacidad de mutación de estado ante hallazgos del enjambre.
// =================================================================

use crate::errors::DbError;
use crate::TursoClient;
use libsql::params;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Entidad de Dominio: Escenario de Prueba ("Golden Ticket").
///
/// Representa un par de claves (Privada/Pública) generado a partir de una entropía conocida,
/// utilizado para validar que el pipeline de minería funciona correctamente.
///
/// Esta estructura es pública para ser consumida directamente por la capa de API (Axum).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestScenario {
    /// Identificador único universal (UUID v4).
    pub id: String,

    /// Nombre descriptivo asignado por el operador (ej: "Debian Bug Simulation").
    pub name: String,

    /// La frase semilla o entropía original (Input secreto).
    pub secret_phrase: String,

    /// La dirección Bitcoin P2PKH esperada (Output público).
    pub target_address: String,

    /// La clave privada WIF esperada (para validación cruzada).
    pub target_private_key: String,

    /// Estado del ciclo de vida: 'idle', 'active', 'verified'.
    pub status: String,

    /// Timestamp de creación (ISO 8601 UTC generado por la DB).
    pub created_at: String,

    /// Timestamp del momento en que un minero encontró este escenario (si aplica).
    pub verified_at: Option<String>,
}

/// Repositorio especializado para la gestión de escenarios de prueba.
pub struct ScenarioRepository {
    client: TursoClient,
}

impl ScenarioRepository {
    /// Constructor: Inyecta el cliente de base de datos para habilitar el pooling.
    pub fn new(client: TursoClient) -> Self {
        Self { client }
    }

    /// Crea un nuevo escenario de prueba de manera atómica.
    ///
    /// # Optimización de Élite
    /// Utiliza la cláusula `RETURNING *` de SQLite/libSQL. Esto permite insertar el registro
    /// y recuperar los valores generados por el servidor (como `created_at` o defaults)
    /// en una sola operación de red, garantizando consistencia absoluta y menor latencia.
    pub async fn create(
        &self,
        name: &str,
        phrase: &str,
        address: &str,
        pk: &str,
    ) -> Result<TestScenario, DbError> {
        let conn = self.client.get_connection()?;
        let id = Uuid::new_v4().to_string();

        let query = r#"
            INSERT INTO test_scenarios
            (id, name, secret_phrase, target_address, target_private_key, status)
            VALUES (?1, ?2, ?3, ?4, ?5, 'idle')
            RETURNING id, name, secret_phrase, target_address, target_private_key, status, created_at, verified_at
        "#;

        let mut rows = conn
            .query(query, params![id, name, phrase, address, pk])
            .await
            .map_err(DbError::QueryError)?;

        // Mapeo inmediato del resultado. Esperamos exactamente 1 fila.
        if let Some(row) = rows.next().await.map_err(DbError::QueryError)? {
            Ok(self.map_row(row)?)
        } else {
            Err(DbError::MappingError(
                "Inconsistencia DB: Insert exitoso pero sin retorno de datos.".to_string(),
            ))
        }
    }

    /// Recupera el inventario completo de escenarios para el Dashboard.
    /// Ordenado por fecha de creación descendente (lo más nuevo primero).
    pub async fn list_all(&self) -> Result<Vec<TestScenario>, DbError> {
        let conn = self.client.get_connection()?;

        let query = r#"
            SELECT id, name, secret_phrase, target_address, target_private_key, status, created_at, verified_at
            FROM test_scenarios
            ORDER BY created_at DESC
        "#;

        let mut rows = conn.query(query, ()).await.map_err(DbError::QueryError)?;
        let mut scenarios = Vec::new();

        while let Some(row) = rows.next().await.map_err(DbError::QueryError)? {
            scenarios.push(self.map_row(row)?);
        }

        Ok(scenarios)
    }

    /// Busca un escenario específico por su dirección objetivo.
    ///
    /// # Uso: The Interceptor
    /// Esta función es el núcleo de la herramienta de verificación manual y automática.
    /// Permite saber instantáneamente si una dirección encontrada pertenece a un experimento controlado.
    pub async fn find_by_address(&self, address: &str) -> Result<Option<TestScenario>, DbError> {
        let conn = self.client.get_connection()?;

        let query = r#"
            SELECT id, name, secret_phrase, target_address, target_private_key, status, created_at, verified_at
            FROM test_scenarios
            WHERE target_address = ?1
            LIMIT 1
        "#;

        let mut rows = conn
            .query(query, params![address])
            .await
            .map_err(DbError::QueryError)?;

        if let Some(row) = rows.next().await.map_err(DbError::QueryError)? {
            Ok(Some(self.map_row(row)?))
        } else {
            Ok(None)
        }
    }

    /// Actualiza el estado de un escenario a "VERIFICADO".
    ///
    /// # Uso: Loop Closure
    /// Se invoca cuando un Worker reporta un hallazgo (`Finding`). Si el hallazgo coincide
    /// con un escenario de prueba, este método sella el experimento, probando empíricamente
    /// que la cadena de búsqueda funciona.
    ///
    /// Retorna `true` si se actualizó algún registro (éxito), `false` si no era un escenario conocido.
    pub async fn mark_as_verified(&self, address: &str) -> Result<bool, DbError> {
        let conn = self.client.get_connection()?;

        let query = r#"
            UPDATE test_scenarios
            SET status = 'verified', verified_at = CURRENT_TIMESTAMP
            WHERE target_address = ?1 AND status != 'verified'
        "#;

        let result = conn
            .execute(query, params![address])
            .await
            .map_err(DbError::QueryError)?;

        // Si rows_affected > 0, significa que encontramos y actualizamos el escenario.
        Ok(result > 0)
    }

    /// Helper privado para el mapeo consistente de filas SQL a Structs Rust.
    /// Centraliza la lógica de deserialización para evitar duplicidad y errores.
    fn map_row(&self, row: libsql::Row) -> Result<TestScenario, DbError> {
        Ok(TestScenario {
            id: row.get(0).map_err(DbError::QueryError)?,
            name: row.get(1).map_err(DbError::QueryError)?,
            secret_phrase: row.get(2).map_err(DbError::QueryError)?,
            target_address: row.get(3).map_err(DbError::QueryError)?,
            target_private_key: row.get(4).map_err(DbError::QueryError)?,
            status: row.get(5).map_err(DbError::QueryError)?,
            created_at: row.get(6).map_err(DbError::QueryError)?,
            // verified_at es opcional (NULLABLE en DB)
            verified_at: row.get(7).ok(),
        })
    }
}
