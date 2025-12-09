
// =================================================================
// APARATO: ORCHESTRATOR MIDDLEWARE
// RESPONSABILIDAD: SEGURIDAD PERIMETRAL (AUTH)
// =================================================================

use axum::{
    extract::Request,
    http::{StatusCode, header},
    middleware::Next,
    response::Response,
};
use tracing::warn;

/// Verifica que la petición tenga el header `Authorization: Bearer <TOKEN>`.
pub async fn auth_guard(req: Request, next: Next) -> Result<Response, StatusCode> {
    // 1. Obtener el token maestro de las variables de entorno
    let secret = std::env::var("WORKER_AUTH_TOKEN")
        .unwrap_or_else(|_| "prospector_insecure_dev_token".to_string());

    // 2. Extraer el header de autorización
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    // 3. Validar formato "Bearer <TOKEN>"
    match auth_header {
        Some(auth_header) if auth_header.starts_with("Bearer ") => {
            let current_token = &auth_header[7..]; // Quitar "Bearer "

            if current_token == secret {
                // ✅ Token válido: Pasar al siguiente handler
                Ok(next.run(req).await)
            } else {
                // ❌ Token incorrecto
                warn!("Intento de acceso no autorizado: Token inválido");
                Err(StatusCode::UNAUTHORIZED)
            }
        }
        _ => {
            // ❌ Header faltante o malformado
            warn!("Intento de acceso no autorizado: Header faltante");
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}
