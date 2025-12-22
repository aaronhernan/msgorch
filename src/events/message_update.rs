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
            //debug!("Payload: {:?}", data);
            return StatusCode::OK;
        }
    };

    // No voy a utilizar idempotencia en updates, por que pueden llegar varios
    // if !state.idempotency.check_and_mark(message_id).await.unwrap_or(false) {
    //     info!("Update duplicado ignorado: {}", message_id);
    //     return StatusCode::OK;
    // }
    let message_id = parsed.key.id.as_deref().unwrap_or("N/A");
    let jid = parsed.key.remote_jid.as_deref().unwrap_or("N/A");
    let status = parsed.update.status.unwrap_or(-1);
    info!(
        message_id = %message_id,
        jid = %jid,
        status = %status,
        "Mensaje actualizado"
    );
    StatusCode::OK
}

    

    