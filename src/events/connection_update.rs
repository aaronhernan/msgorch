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
    let _parsed: ConnectionUpdateData = match serde_json::from_value(data) {
        Ok(v) => v,
        Err(err) => {
            warn!("connection.update malformado: {}", err);
            return StatusCode::OK;
        }
    };

    info!("Connection update recibido");

    StatusCode::OK
}