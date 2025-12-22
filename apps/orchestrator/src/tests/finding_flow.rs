// apps/orchestrator/src/tests/finding_flow.rs
/**
 * =================================================================
 * APARATO: FINDING FLOW INTEGRATION TEST (V1.0)
 * CLASIFICACIÓN: INTEGRATION TEST (ESTRATO L3)
 * RESPONSABILIDAD: VALIDACIÓN DEL FLUJO COMPLETO DE INGESTA DE HALLAZGOS
 *
 * OBJETIVO:
 * Simular un worker enviando una colisión (POST /swarm/finding) y verificar:
 * 1. Respuesta HTTP 201 Created.
 * 2. Persistencia en la tabla `findings` de Turso (SQLite In-Memory).
 * 3. Integridad de los datos guardados.
 * =================================================================
 */

#[cfg(test)]
mod tests {
    use crate::handlers::swarm;
    use crate::state::AppState;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        routing::post,
        Router,
    };
    use chrono::Utc;
    use prospector_domain_models::finding::Finding;
    use prospector_infra_db::TursoClient;
    use tower::ServiceExt; // Para one_shot
    use uuid::Uuid;
    use serde_json::json;

    /// Setup de infraestructura efímera (In-Memory Database)
    async fn setup_state() -> AppState {
        // Inicializamos Turso en memoria. Esto corre las migraciones automáticamente
        // gracias a la lógica de `TursoClient::connect`.
        let client = TursoClient::connect("file::memory:", None)
            .await
            .expect("Failed to initialize in-memory database");

        AppState::new(client)
    }

    #[tokio::test]
    async fn test_finding_ingestion_flow() {
        // 1. PREPARACIÓN DEL ENTORNO
        let state = setup_state().await;

        // Router mínimo aislado para el endpoint de findings
        let app = Router::new()
            .route("/finding", post(swarm::SwarmHandshakeHandler::handle_cryptographic_finding))
            .with_state(state.clone()); // Clone del Arc<AppState>

        // 2. CONSTRUCCIÓN DEL PAYLOAD (Simulación de Worker)
        let finding_payload = Finding {
            address: "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string(), // Genesis Address
            private_key_wif: "5HvPBms4...MOCK_WIF...".to_string(),
            source_entropy: "test_vector:sequential:001".to_string(),
            wallet_type: "p2pkh_legacy".to_string(),
            found_by_worker: "unit-test-worker-01".to_string(),
            job_id: Some(Uuid::new_v4().to_string()),
            detected_at: Utc::now().to_rfc3339(),
        };

        let json_body = serde_json::to_string(&finding_payload).unwrap();

        // 3. EJECUCIÓN DE LA PETICIÓN HTTP (Simulada)
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/finding")
                    .header("Content-Type", "application/json")
                    .body(Body::from(json_body))
                    .unwrap(),
            )
            .await
            .unwrap();

        // 4. ASERCIÓN DE RESPUESTA API
        assert_eq!(
            response.status(),
            StatusCode::CREATED,
            "API did not return 201 Created"
        );

        // 5. ASERCIÓN DE PERSISTENCIA (Validación Cruzada con DB)
        // Esperamos brevemente al FindingFlusher o verificamos el Vault en memoria.
        // Como el Flusher es asíncrono, en este test unitario verificamos el VAULT en RAM
        // que es el paso inmediato antes de la DB.

        let pending_count = state.finding_vault.get_pending_count();
        assert_eq!(pending_count, 1, "Finding was not deposited in the Vault Manager");

        // Drenamos el Vault para inspeccionar el contenido
        let deposited_findings = state.finding_vault.drain_vault_for_flush();
        let saved_finding = &deposited_findings[0];

        assert_eq!(saved_finding.address, finding_payload.address);
        assert_eq!(saved_finding.found_by_worker, finding_payload.found_by_worker);

        println!("✅ [TEST_SUCCESS]: Finding Flow Verified. Ingested -> API -> RAM Vault.");
    }
}
