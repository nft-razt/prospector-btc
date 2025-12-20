/**
 * =================================================================
 * APARATO: MISSION REPOSITORY (V125.0 - SOBERANO)
 * CLASIFICACIÓN: INFRASTRUCTURE ADAPTER (ESTRATO L3)
 * RESPONSABILIDAD: SECUENCIACIÓN DE UPTIME Y RECUPERACIÓN DE ERRORES
 * =================================================================
 */

use crate::errors::DbError;
use libsql::{params, Connection};
use prospector_domain_models::work::{WorkOrder, SearchStrategy};
use uuid::Uuid;

pub struct MissionRepository;

impl MissionRepository {
    /**
     * Adquiere misiones de simulación XP garantizando la cobertura total del tiempo.
     * Implementa recuperación automática de misiones abandonadas.
     */
    pub async fn acquire_dynamic_mission_atomic(
        connection: &Connection,
        worker_id: &str,
        scenario_id: &str,
        clock_frequency: u64
    ) -> Result<WorkOrder, DbError> {
        let transaction = connection.transaction().await?;

        // 1. RECLAMACIÓN DE MISIONES HUÉRFANAS (Auto-Regeneración)
        let orphan_query = "SELECT id, uptime_seconds_start, uptime_seconds_end FROM jobs
                            WHERE status = 'active' AND (unixepoch() - unixepoch(started_at)) > 900 LIMIT 1";

        let mut rows = transaction.query(orphan_query, ()).await?;
        if let Some(row) = rows.next().await? {
            let mission_id: String = row.get(0)?;
            transaction.execute("UPDATE jobs SET worker_id = ?1, started_at = CURRENT_TIMESTAMP WHERE id = ?2",
                params![worker_id, mission_id.clone()]).await?;
            transaction.commit().await?;

            return Ok(WorkOrder {
                job_mission_identifier: mission_id,
                lease_duration_seconds: 900,
                strategy: SearchStrategy::SatoshiWindowsXpForensic {
                    scenario_template_identifier: scenario_id.to_string(),
                    uptime_seconds_start: row.get(1)?,
                    uptime_seconds_end: row.get(2)?,
                    hardware_clock_frequency: clock_frequency,
                },
            });
        }

        // 2. EXPANSIÓN DE FRONTERA TEMPORAL
        let frontier_query = "SELECT MAX(uptime_seconds_end) FROM jobs WHERE scenario_template_identifier = ?1";
        let mut frontier_rows = transaction.query(frontier_query, params![scenario_id]).await?;
        let last_second: u64 = frontier_rows.next().await?.and_then(|r| r.get(0).ok()).unwrap_or(0);

        let next_start = last_second;
        let next_end = next_start + 60; // Bloques de 1 minuto
        let new_uuid = Uuid::new_v4().to_string();

        transaction.execute(
            "INSERT INTO jobs (id, scenario_template_identifier, uptime_seconds_start, uptime_seconds_end,
             hardware_clock_frequency, worker_id, status, strategy_type)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, 'active', 'SatoshiWindowsXpForensic')",
            params![new_uuid.clone(), scenario_id, next_start, next_end, clock_frequency, worker_id]
        ).await?;

        transaction.commit().await?;

        Ok(WorkOrder {
            job_mission_identifier: new_uuid,
            lease_duration_seconds: 900,
            strategy: SearchStrategy::SatoshiWindowsXpForensic {
                scenario_template_identifier: scenario_id.to_string(),
                uptime_seconds_start: next_start,
                uptime_seconds_end: next_end,
                hardware_clock_frequency: clock_frequency,
            },
        })
    }
}
