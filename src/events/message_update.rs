use axum::http::StatusCode;
use serde_json::Value;
use tracing::{info, warn};

use crate::{
    app::AppState, 
    models::evolution::message_update::MessageUpdateData
};

pub async fn handle(_state: &AppState, data: Value) -> StatusCode {
    let parsed: MessageUpdateData = match serde_json::from_value(data) {
        Ok(v) => v,
        Err(err) => {
            warn!("Error parseando message_update: {}", err);
            return StatusCode::OK;
        }
    };

    info!(
        transporter_id = %parsed.key_id,
        jid = %parsed.remote_jid,
        status = %parsed.status.unwrap_or("<no value>".to_string()),
        "Mensaje actualizado"
    );
    StatusCode::OK
}

    

    