/**
 * =================================================================
 * APARATO: MISSION STRATEGIC REPOSITORY (V221.0 - ZERO WARNINGS)
 * CLASIFICACIÓN: INFRASTRUCTURE ADAPTER (ESTRATO L3)
 * RESPONSABILIDAD: GESTIÓN TRANSACCIONAL DEL CICLO DE VIDA DE MISIONES
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa el motor de despacho sensible al hardware. Esta versión
 * elimina los ruidos de compilación (unused macros) manteniendo
 * la integridad de la lógica de recuperación de zombies y despacho.
 * =================================================================
 */

use crate::errors::DbError;
use crate::TursoClient;
use libsql::{params, Connection, Row};
use prospector_domain_models::work::{
    WorkOrder, SearchStrategy, TargetStrata, AuditReport
};
use uuid::Uuid;
use tracing::{info, instrument}; // ✅ RESOLUCIÓN: Eliminado 'warn' y 'error' no utilizados

pub struct MissionRepository {
    database_client: TursoClient,
}

impl MissionRepository {
    pub fn new(client: TursoClient) -> Self {
        Self { database_client: client }
    }

    /**
     * Recupera y reclama una misión adecuada para las capacidades del trabajador.
     */
    #[instrument(skip(self))]
    pub async fn fetch_intelligent_assignment(
        &self,
        worker_id: &str,
        available_ram_megabytes: u64
    ) -> Result<Option<WorkOrder>, DbError> {
        let connection = self.database_client.get_connection()?;
        let transaction = connection.transaction().await?;

        // 1. DETERMINACIÓN DE ESTRATO COMPATIBLE
        let preferred_strata = if available_ram_megabytes >= 8192 {
            TargetStrata::SatoshiEra
        } else {
            TargetStrata::VulnerableLegacy
        };

        let strata_label = match preferred_strata {
            TargetStrata::SatoshiEra => "SatoshiEra",
            _ => "VulnerableLegacy",
        };

        // 2. BÚSQUEDA DE MISIÓN DISPONIBLE
        let select_query = "
            SELECT id, range_start, range_end, strategy_type, scenario_template_identifier,
                   uptime_seconds_start, uptime_seconds_end, hardware_clock_frequency, required_strata
            FROM jobs
            WHERE status = 'queued' AND required_strata = ?1
            LIMIT 1
        ";

        let mut rows = transaction.query(select_query, params![strata_label]).await?;

        if let Some(row) = rows.next().await? {
            let mission_identifier: String = row.get(0)?;

            // 3. RECLAMO ATÓMICO
            transaction.execute(
                "UPDATE jobs SET status = 'active', worker_id = ?1, updated_at = CURRENT_TIMESTAMP WHERE id = ?2",
                params![worker_id, mission_identifier.clone()]
            ).await?;

            let work_order = self.map_row_to_work_order(&row)?;
            transaction.commit().await?;

            info!("🎯 [DISPATCH]: Mission {} assigned to node {}.", mission_identifier, worker_id);
            Ok(Some(work_order))
        } else {
            Ok(None)
        }
    }

    #[instrument(skip(self))]
    pub async fn fetch_dynamic_mission_batch(&self, volume: usize) -> Result<Vec<WorkOrder>, DbError> {
        let connection = self.database_client.get_connection()?;
        let mut config_rows = connection.query("SELECT value_text, value_int FROM system_state WHERE key = 'active_scenario_config'", ()).await?;

        let (active_scenario_id, clock_frequency) = if let Some(row) = config_rows.next().await? {
            (row.get::<String>(0)?, row.get::<i64>(1)? as u64)
        } else {
            ("WIN_XP_SP3_GOLD".to_string(), 3579545u64)
        };

        self.pre_allocate_missions_atomic(&active_scenario_id, volume, clock_frequency).await
    }

    async fn pre_allocate_missions_atomic(&self, scenario_id: &str, volume: usize, frequency: u64) -> Result<Vec<WorkOrder>, DbError> {
        let connection = self.database_client.get_connection()?;
        let transaction = connection.transaction().await?;
        let mut rows = transaction.query("SELECT MAX(uptime_seconds_end) FROM jobs WHERE scenario_template_identifier = ?1", params![scenario_id]).await?;

        let mut current_boundary: u64 = rows.next().await?
            .and_then(|row| row.get::<Option<i64>>(0).ok())
            .flatten()
            .map(|v| v as u64)
            .unwrap_or(0);

        let mut batch = Vec::with_capacity(volume);
        for _ in 0..volume {
            let start = current_boundary;
            let end = start + 60;
            let id = Uuid::new_v4().to_string();

            transaction.execute(
                "INSERT INTO jobs (id, scenario_template_identifier, uptime_seconds_start, uptime_seconds_end,
                 hardware_clock_frequency, status, strategy_type, required_strata, range_start, range_end)
                 VALUES (?1, ?2, ?3, ?4, ?5, 'queued', 'SatoshiWindowsXpForensic', 'SatoshiEra', '0', '0')",
                params![id.clone(), scenario_id, start as i64, end as i64, frequency as i64]
            ).await?;

            batch.push(WorkOrder {
                job_mission_identifier: id,
                lease_duration_seconds: 900,
                strategy: SearchStrategy::SatoshiWindowsXpForensic {
                    scenario_template_identifier: scenario_id.to_string(),
                    uptime_seconds_start: start,
                    uptime_seconds_end: end,
                    hardware_clock_frequency: frequency,
                },
                required_strata: TargetStrata::SatoshiEra,
            });
            current_boundary = end;
        }
        transaction.commit().await?;
        Ok(batch)
    }

    pub async fn identify_abandoned_missions(&self, conn: &Connection, timeout: i64) -> Result<Vec<String>, DbError> {
        let mut rows = conn.query("SELECT id FROM jobs WHERE status = 'active' AND (unixepoch() - unixepoch(updated_at)) > ?1", params![timeout]).await?;
        let mut identifiers = Vec::new();
        while let Some(row) = rows.next().await? { identifiers.push(row.get::<String>(0)?); }
        Ok(identifiers)
    }

    pub async fn requeue_missions(&self, conn: &Connection, mission_ids: Vec<String>) -> Result<(), DbError> {
        if mission_ids.is_empty() { return Ok(()); }
        let transaction = conn.transaction().await?;
        for id in mission_ids {
            transaction.execute("UPDATE jobs SET status = 'queued', worker_id = 'unassigned', updated_at = CURRENT_TIMESTAMP WHERE id = ?1", params![id]).await?;
        }
        transaction.commit().await?;
        Ok(())
    }

    pub async fn certify_mission_completion(&self, report: &AuditReport) -> Result<(), DbError> {
        let connection = self.database_client.get_connection()?;
        connection.execute(
            "UPDATE jobs SET status = 'completed', total_hashes_effort = ?2, audit_footprint_checkpoint = ?3, execution_duration_ms = ?4, completed_at = CURRENT_TIMESTAMP WHERE id = ?1",
            params![report.job_mission_identifier.clone(), report.computational_effort_volume.clone(), report.audit_footprint_checkpoint.clone(), report.execution_duration_milliseconds as i64]
        ).await?;
        Ok(())
    }

    fn map_row_to_work_order(&self, row: &Row) -> Result<WorkOrder, DbError> {
        let strategy_label: String = row.get(3)?;
        let strata_label: String = row.get(8).unwrap_or_else(|_| "SatoshiEra".to_string());
        let strategy = match strategy_label.as_str() {
            "SatoshiWindowsXpForensic" => SearchStrategy::SatoshiWindowsXpForensic {
                scenario_template_identifier: row.get(4)?,
                uptime_seconds_start: row.get::<i64>(5)? as u64,
                uptime_seconds_end: row.get::<i64>(6)? as u64,
                hardware_clock_frequency: row.get::<i64>(7)? as u64,
            },
            _ => SearchStrategy::Sequential { start_index_hexadecimal: row.get(1)?, end_index_hexadecimal: row.get(2)? },
        };
        let required_strata = match strata_label.as_str() {
            "VulnerableLegacy" => TargetStrata::VulnerableLegacy,
            "StandardLegacy" => TargetStrata::StandardLegacy,
            _ => TargetStrata::SatoshiEra,
        };
        Ok(WorkOrder { job_mission_identifier: row.get(0)?, lease_duration_seconds: 900, strategy, required_strata })
    }
}
