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
        chat_id = %parsed.id,
        unread = ?parsed.unread_count,
        archived = ?parsed.archived,
        pinned = ?parsed.pinned,
        name = ?parsed.name,
        "Chat actualizado"
    );

    StatusCode::OK
}