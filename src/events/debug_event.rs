use axum::http::StatusCode;
use serde_json::Value;
use tracing::{info};

use crate::{
    app::AppState,
};

pub async fn handle(_state: &AppState, data: Value) -> StatusCode {
    info!("Evento de depuración recibido con datos: {:?}", data);
    StatusCode::OK
}