use axum::http::StatusCode;
use serde_json::Value;
use tracing::{info, warn};

use crate::{
    models::evolution::chat_update::ChatUpdateData,
    app::AppState,
};

pub async fn handle(
    _state: &AppState,
    data: Value,
) -> StatusCode {
    let parsed: ChatUpdateData = match serde_json::from_value(data) {
        Ok(v) => v,
        Err(err) => {
            warn!("No se pudo deserializar chats.update: {}", err);
            return StatusCode::OK;
        }
    };

    info!(
        remote_jid = %parsed.remote_jid,
        instance_id = %parsed.instance_id,
        "Chat actualizado"
    );

    StatusCode::OK
}