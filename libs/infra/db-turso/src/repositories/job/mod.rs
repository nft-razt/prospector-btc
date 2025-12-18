// libs/infra/db-turso/src/repositories/job/mod.rs
// =================================================================
// APARATO: JOB REPOSITORY (V16.0 - ATOMIC LEDGER GUARD)
// CLASIFICACIÓN: INFRASTRUCTURE LAYER (L3)
// RESPONSABILIDAD: GESTIÓN DEL CICLO DE VIDA DE ÓRDENES DE TRABAJO
//
// CARACTERÍSTICAS DE ÉLITE:
// - Transaccionalidad ACID: Garantiza que un rango sea asignado a un solo worker.
// - Recuperación de Huérfanos: Algoritmo de detección y reasignación de 'Zombies'.
// - Soberanía U256: Cálculo de fronteras delegada al motor matemático nivelado.
// =================================================================

pub mod math;
pub mod queries;

use anyhow::{anyhow, Result};
use chrono::{Duration, Utc};
use libsql::{params, Connection};
use tracing::{info, warn, instrument};
use uuid::Uuid;

// Importaciones de módulos locales nivelados
use self::math::RangeCalculator;
use self::queries as sql;

// Modelos de dominio compartidos
use prospector_domain_models::{SearchStrategy, WorkOrder};

/// Tiempo de inactividad permitido (en minutos) antes de que un trabajo
/// sea marcado como huérfano y reasignado a la red.
const ZOMBIE_INACTIVITY_THRESHOLD_MINUTES: i64 = 10;

/// Repositorio de persistencia para la gestión del Ledger Táctico.
pub struct JobRepository {
    database_connection: Connection,
}

impl JobRepository {
    /// Inicializa una nueva instancia del repositorio inyectando la conexión activa.
    pub fn new(connection: Connection) -> Self {
        Self {
            database_connection: connection,
        }
    }

    /// Orquesta la asignación de una unidad de trabajo para un nodo del enjambre.
    ///
    /// # Flujo Logístico
    /// 1. Busca trabajos 'Zombies' (estancados) o 'Pendientes' para cerrar huecos en el ledger.
    /// 2. Si no hay remanentes, expande la frontera de búsqueda calculando el siguiente rango U256.
    /// 3. Ejecuta la operación dentro de una transacción exclusiva para evitar colisiones de asignación.
    #[instrument(skip(self, worker_identifier))]
    pub async fn assign_to_worker(&self, worker_identifier: &str) -> Result<WorkOrder> {
        let expiration_timestamp = Utc::now() - Duration::minutes(ZOMBIE_INACTIVITY_THRESHOLD_MINUTES);
        let transaction = self.database_connection.transaction().await?;

        // --- FASE 1: RECUPERACIÓN DE TRABAJOS HUÉRFANOS ---
        let mut recoverable_jobs_result = transaction.query(
            sql::FIND_RECOVERABLE_JOB,
            params![expiration_timestamp.to_rfc3339()],
        ).await?;

        if let Some(row) = recoverable_jobs_result.next().await? {
            let job_id: String = row.get(0)?;
            let range_start: String = row.get(1)?;
            let range_end: String = row.get(2)?;

            transaction.execute(
                sql::CLAIM_JOB,
                params![worker_identifier, job_id.clone()]
            ).await?;

            transaction.commit().await?;

            info!(
                target: "prospector::infra",
                "♻️  RECOVERY: Job [{}] reassigned to worker [{}]",
                job_id,
                worker_identifier
            );

            return Ok(self.map_to_domain_order(job_id, range_start, range_end));
        }

        // --- FASE 2: EXPANSIÓN DEL ESPACIO DE BÚSQUEDA ---
        // Consultamos la frontera actual del Ledger Táctico.
        let mut boundary_result = transaction.query(sql::GET_LAST_EXPLORED_BOUNDARY, ()).await?;
        let last_boundary_hex = boundary_result.next().await?
            .and_then(|row| row.get::<String>(0).ok());

        // El motor matemático (RangeCalculator) determina los próximos bytes de inicio y fin.
        let (next_start_hex, next_end_hex) = RangeCalculator::calculate_next(last_boundary_hex)?;

        let new_job_uuid = Uuid::new_v4().to_string();

        transaction.execute(
            sql::INITIALIZE_JOB,
            params![
                new_job_uuid.clone(),
                next_start_hex.clone(),
                next_end_hex.clone(),
                worker_identifier
            ],
        ).await?;

        transaction.commit().await?;

        info!(
            target: "prospector::infra",
            "✨  EXPANSION: New range segment [{}] deployed to worker [{}]",
            new_job_uuid,
            worker_identifier
        );

        Ok(self.map_to_domain_order(new_job_uuid, next_start_hex, next_end_hex))
    }

    /// Registra el pulso de actividad de un trabajo, extendiendo su tiempo de vida.
    ///
    /// Previene que el servicio 'Reaper' reclame el trabajo mientras el worker está operando.
    pub async fn report_progress_heartbeat(&self, job_identifier: &str) -> Result<()> {
        let rows_affected = self.database_connection
            .execute(sql::UPDATE_HEARTBEAT, params![job_identifier])
            .await?;

        if rows_affected == 0 {
            warn!("⚠️  HEARTBEAT_REJECTED: Job [{}] is not registered in active strata.", job_identifier);
            return Err(anyhow!("Target job not found in tactical persistence."));
        }

        Ok(())
    }

    /// Sella un trabajo como finalizado exitosamente.
    ///
    /// Este paso es indispensable para que el puente hacia Supabase (L4)
    /// reconozca el trabajo como apto para migración estratégica.
    pub async fn finalize_job_success(&self, job_identifier: &str) -> Result<()> {
        let rows_affected = self.database_connection
            .execute(sql::MARK_COMPLETED, params![job_identifier])
            .await?;

        if rows_affected == 0 {
            return Err(anyhow!("FATAL: Attempted to complete a non-existent job sequence."));
        }

        Ok(())
    }

    /// Helper de mapeo interno para la construcción del contrato de dominio.
    ///
    /// Transforma los datos crudos de persistencia en una orden de trabajo
    /// procesable por el motor de minería (StrategyExecutor).
    fn map_to_domain_order(&self, identifier: String, start_hex: String, end_hex: String) -> WorkOrder {
        WorkOrder {
            id: identifier,
            // Duración objetivo para que el worker reporte antes de expirar
            target_duration_sec: 600,
            strategy: SearchStrategy::Combinatoric {
                prefix: "".to_string(), // Dinámico en futuras campañas
                suffix: "".to_string(),
                start_index: start_hex,
                end_index: end_hex,
            },
        }
    }
}
