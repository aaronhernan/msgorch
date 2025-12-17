use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    body::Body,
};
use tracing::warn;

use crate::{
    config::Config,
    services::evolution::EvolutionService,
};

pub async fn webhook_auth(
    State((config, _evolution)): State<(Config, EvolutionService)>,
    req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = req
        .headers()
        .get("x-webhook-token")
        .and_then(|v| v.to_str().ok());

    match token {
        Some(t) if t == config.webhook_token => {
            Ok(next.run(req).await)
        }
        _ => {
            warn!("Webhook rechazado por token inválido");
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}