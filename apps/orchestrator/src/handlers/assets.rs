/**
 * =================================================================
 * APARATO: ASSET GATEWAY (V30.0 - STRATA SUPPORT)
 * RESPONSABILIDAD: SERVIDO DE FRAGMENTOS BINARIOS POR ERA
 * =================================================================
 */

use crate::state::AppState;
use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse};
use std::path::PathBuf;
use tokio::fs;

pub struct AssetGatewayHandler;

impl AssetGatewayHandler {
    /**
     * Endpoint: GET /api/v1/assets/dna/:strata/:filename
     * Sirve el fragmento binario solicitado desde el disco del Orquestador.
     */
    pub async fn download_shard(
        State(_): State<AppState>,
        Path((strata, filename)): Path<(String, String)>,
    ) -> impl IntoResponse {
        // Validación de seguridad de ruta (Evitar Path Traversal)
        if filename.contains("..") || strata.contains("..") {
            return StatusCode::FORBIDDEN.into_response();
        }

        // Construcción de la ruta hacia el almacenamiento de filtros (L1-ETL output)
        let base_path = PathBuf::from("dist/filters");
        let full_path = base_path.join(strata).join(filename);

        match fs::read(&full_path).await {
            Ok(content) => {
                (
                    [
                        ("Content-Type", "application/octet-stream"),
                        ("X-Strata-Type", "CHRONO_BLOOM"),
                    ],
                    content
                ).into_response()
            },
            Err(_) => StatusCode::NOT_FOUND.into_response(),
        }
    }
}
