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
    use prospector_domain_models::WorkerHeartbeat;
    use prospector_infra_db::TursoClient;
    use tower::ServiceExt; // para one_shot
    use uuid::Uuid;

    // Helper para levantar un estado con DB en memoria (Volátil)
    async fn setup_state() -> AppState {
        // "file::memory:" crea una DB SQLite aislada en RAM
        let client = TursoClient::connect("file::memory:", None).await.unwrap();
        AppState::new(client)
    }

    #[tokio::test]
    async fn test_heartbeat_handler() {
        let state = setup_state().await;

        // Creamos un router mínimo aislado solo para este test
        let app = Router::new()
            .route("/heartbeat", post(swarm::receive_heartbeat))
            .with_state(state);

        let heartbeat = WorkerHeartbeat {
            worker_id: Uuid::new_v4(),
            hostname: "test-unit-alpha".to_string(),
            hashrate: 5000,
            current_job_id: None,
            timestamp: Utc::now(),
        };

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/heartbeat")
                    .header("Content-Type", "application/json")
                    .body(Body::from(serde_json::to_string(&heartbeat).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
