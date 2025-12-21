use tracing::warn;
use axum::http::StatusCode;

use crate::{
    app::AppState,
    models::webhook::WebhookEnvelope,
    events::message_upsert,
};

pub async fn dispatch( payload: WebhookEnvelope, state: &AppState, ) -> StatusCode {
    match payload.event.as_str() {
        "messages.upsert" => {
            message_upsert::handle(state, payload.data).await
        }

        other => {
            warn!("Evento no manejado: {}", other);
            StatusCode::OK
        }
    }
}