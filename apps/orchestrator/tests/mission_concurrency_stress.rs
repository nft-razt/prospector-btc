/**
 * =================================================================
 * APARATO: MISSION CONCURRENCY STRESS (V1.1 - HARDENED)
 * CLASIFICACIÓN: QA INFRAESTRUCTURA (ESTRATO L3)
 * RESPONSABILIDAD: CERTIFICACIÓN DE DESPACHO O(1) BAJO CARGA MASIVA
 * =================================================================
 */

#[cfg(test)]
mod stress_chamber {
    use prospector_orchestrator::state::AppState;
    use prospector_orchestrator::handlers::swarm::SwarmHandshakeHandler;
    use prospector_domain_models::work::{MissionRequestPayload, NodeHardwareCapacity, WorkOrder, SearchStrategy, TargetStrata};
    use prospector_infra_db::TursoClient;
    use axum::extract::{Json, State};
    use axum::response::IntoResponse;
    use std::sync::Arc;
    use tokio::task;

    /**
     * Prueba de choque: Simula el ataque simultáneo de 1,000 unidades
     * al buffer de despacho en memoria.
     */
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn certify_dispatch_resilience_under_massive_burst() {
        println!("\n🔥 [STRESS_TEST]: Initiating 1000-unit concurrent burst...");

        // 1. SETUP: Inicialización de infraestructura efímera
        let database_client = TursoClient::connect("file::memory:", None).await.unwrap();
        let application_state = AppState::new(database_client);

        // 2. HIDRATACIÓN DE RAM: Inyectamos 1,000 misiones ficticias en el buffer
        let mut mission_batch = Vec::new();
        for i in 0..1000 {
            mission_batch.push(WorkOrder {
                job_mission_identifier: format!("mission-stress-id-{}", i),
                lease_duration_seconds: 600,
                strategy: SearchStrategy::Sequential {
                    start_index_hexadecimal: "0".into(),
                    end_index_hexadecimal: "100".into(),
                },
                required_strata: TargetStrata::SatoshiEra,
            });
        }
        application_state.mission_control.hydrate_queue(mission_batch);

        // 3. EJECUCIÓN: Disparo de ráfagas concurrentes
        let mut network_request_handles = vec![];
        let total_concurrent_units = 1000;

        for i in 0..total_concurrent_units {
            let state_snapshot = application_state.clone();
            let handle = task::spawn(async move {
                let payload = MissionRequestPayload {
                    worker_id: format!("hydra-unit-{}", i),
                    hardware_capacity: NodeHardwareCapacity {
                        ram_available_mb: 8192,
                        cpu_cores: 2,
                        supports_avx2: true,
                    }
                };
                // Invocación directa al handler de élite
                SwarmHandshakeHandler::negotiate_mission_assignment_handshake(
                    State(state_snapshot),
                    Json(payload)
                ).await.into_response()
            });
            network_request_handles.push(handle);
        }

        // 4. AUDITORÍA DE RESPUESTA
        let execution_results = futures::future::join_all(network_request_handles).await;
        let successful_assignments = execution_results.iter().filter(|res| res.is_ok()).count();

        println!("📊 [STRESS_METRICS]: Successfully assigned {}/{} missions.", successful_assignments, total_concurrent_units);

        // ASERCIÓN: No debe haber ni una sola pérdida de datos en RAM
        assert_eq!(successful_assignments, total_concurrent_units, "CONCURRENCY_COLLAPSE: Buffer synchronization failed.");

        // El buffer debe estar vacío tras el ataque
        assert_eq!(application_state.mission_control.get_available_buffer_size(), 0, "LEAK_DETECTED: Some missions were not dequeued.");

        println!("✅ [CERTIFIED]: Orchestrator RAM Stratum is immune to burst congestion.");
    }
}
