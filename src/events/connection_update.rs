use axum::http::StatusCode;
use serde_json::Value;
use tracing::{info, warn};

use crate::{
    app::AppState,
    models::evolution::connection_update::ConnectionUpdateData,
};

pub async fn handle(
    _state: &AppState,
    data: Value,
) -> StatusCode {
    let parsed: ConnectionUpdateData = match serde_json::from_value(data) {
        Ok(v) => v,
        Err(err) => {
            warn!("connection.update malformado: {}", err);
            return StatusCode::OK;
        }
    };

    info!(
        instance = parsed.instance.as_deref().unwrap_or("unknown"),
        state = parsed.state.as_deref().unwrap_or("unknown"),
        status = parsed.status.as_deref().unwrap_or("unknown"),
        reason = parsed.reason.as_deref().unwrap_or(""),
        "Connection update recibido"
    );

    StatusCode::OK
}