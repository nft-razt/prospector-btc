/**
 * =================================================================
 * APARATO: ATOMIC MISSION SEQUENCER (V32.0 - SWARM SAFE)
 * CLASIFICACIÓN: INFRASTRUCTURE ADAPTER (L3)
 * RESPONSABILIDAD: ADQUISICIÓN DE RANGOS SIN COLISIONES
 * =================================================================
 */

use crate::errors::DbError;
use crate::TursoClient;
use libsql::{params, Connection};
use prospector_core_math::prelude::*;
use prospector_domain_models::work::{WorkOrder, SearchStrategy};
use tracing::{info, instrument};
use uuid::Uuid;

pub struct MissionRepository {
    database_connection: Connection,
}

impl MissionRepository {
    pub fn new(connection: Connection) -> Self {
        Self { database_connection: connection }
    }

    /**
     * Adquiere la siguiente misión garantizando exclusividad.
     * Implementa un 'Write Barrier' para evitar que dos nodos minen el mismo rango.
     */
    #[instrument(skip(self, worker_node_identifier))]
    pub async fn acquire_next_mission_atomic(
        &self,
        worker_node_identifier: &str
    ) -> Result<WorkOrder, DbError> {
        // FASE 1: BLOQUEO ESTRATÉGICO
        // Utilizamos una transacción exclusiva en Turso para aislar el cálculo de la frontera.
        let database_transaction = self.database_connection.transaction().await?;

        // FASE 2: DETERMINACIÓN DE FRONTERA
        let mut rows = database_transaction.query(
            "SELECT range_end_hex FROM jobs WHERE status != 'error' ORDER BY range_end_hex DESC LIMIT 1",
            ()
        ).await?;

        let last_boundary_hex = if let Some(row) = rows.next().await? {
            row.get::<String>(0).unwrap_or_else(|_| "0".repeat(64))
        } else {
            "0".repeat(64)
        };

        // FASE 3: INCREMENTO SOBERANO (U256)
        let mut boundary_bytes = [0u8; 32];
        hex::decode_to_slice(&last_boundary_hex, &mut boundary_bytes).unwrap_or_default();

        let mission_start_bytes = add_u64_to_u256_be(&boundary_bytes, 1)
            .map_err(|_| DbError::MappingError("Universal Space Exhausted".into()))?;

        // Paso constante de 1 Billón por misión
        let mission_end_bytes = add_u64_to_u256_be(&mission_start_bytes, 1_000_000_000)
            .map_err(|_| DbError::MappingError("Search Horizon Overflow".into()))?;

        let mission_start_hex = hex::encode(mission_start_bytes);
        let mission_end_hex = hex::encode(mission_end_bytes);

        // FASE 4: SELLO DE RESERVA
        let mission_uuid = Uuid::new_v4().to_string();
        database_transaction.execute(
            r#"INSERT INTO jobs (id, range_start_hex, range_end_hex, strategy_type, worker_id, status)
               VALUES (?1, ?2, ?3, 'Sequential', ?4, 'active')"#,
            params![
                mission_uuid.clone(),
                mission_start_hex.clone(),
                mission_end_hex.clone(),
                worker_node_identifier
            ]
        ).await?;

        // FASE 5: LIBERACIÓN DEL CANAL
        database_transaction.commit().await?;

        Ok(WorkOrder {
            job_mission_identifier: mission_uuid,
            lease_duration: 600,
            strategy: SearchStrategy::Sequential {
                start_index_hex: mission_start_hex,
                end_index_hex: mission_end_hex,
            },
        })
    }
}
