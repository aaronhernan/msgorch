use axum::http::StatusCode;
use serde_json::Value;
use tracing::warn;

use crate::app::AppState;

pub async fn handle(
    _state: &AppState,
    _data: Value,
) -> StatusCode {
    warn!("messages.delete recibido pero no implementado");
    StatusCode::OK
}