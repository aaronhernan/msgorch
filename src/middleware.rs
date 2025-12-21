use axum::{
    http::{Request, StatusCode, header},
    middleware::Next,
    response::Response,
    extract::State,
};
use tracing::warn;

use crate::{
    app::AppState,
};

pub async fn webhook_auth(
    State(state): State<AppState>,
    req: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok());

    match auth_header {
        Some(value) => {
            // Esperamos: "Bearer TOKEN"
            if let Some(token) = value.strip_prefix("Bearer ") {
                    // Comentario de aprendizaje: 
                    // La utilizacion de state, la hacemos mediante la refencia al state y no clonamos nada.
                    // Esto lo hacemos por que el codigo es local y no se mueve a otro hilo, y no hay escape del lifetime.
                if token == &state.config.webhook_token {
                    return Ok(next.run(req).await);
                }
            }

            warn!("Token inválido recibido en webhook");
            Err(StatusCode::UNAUTHORIZED)
        }

        None => {
            warn!("Webhook sin header Authorization");
            Err(StatusCode::UNAUTHORIZED)
        }
    }

}

// pub async fn webhook_auth_debug(
//     State(state): State<AppState>,
//     req: Request<axum::body::Body>,
//     next: Next,
// ) -> Result<Response, StatusCode> {
//     let auth_header = req
//         .headers()
//         .get(header::AUTHORIZATION)
//         .and_then(|v| v.to_str().ok());

//     tracing::info!("Headers: {:?}", req.headers());
//     return Ok(next.run(req).await);    
// }
