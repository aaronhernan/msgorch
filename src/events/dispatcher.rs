use tracing::warn;
use axum::http::StatusCode;

use crate::{
    app::AppState,
    models::webhook::WebhookEnvelope,
    events,
};

pub async fn dispatch( payload: WebhookEnvelope, state: &AppState, ) -> StatusCode {
    match payload.event.as_str() {
        "messages.upsert"   => { events::message_upsert::handle(state, payload.data).await }
        "messages.update"   => { events::message_update::handle(state, payload.data).await }
        "messages.delete"   => { events::message_delete::handle(state, payload.data).await }
        "connection.update" => { events::connection_update::handle(state, payload.data).await }
        "presence.update"   => { events::presence_update::handle(state, payload.data).await }
        "chats.update"      => { events::chats_update::handle(state, payload.data).await }
        other => {
            warn!("Evento no manejado: {}", other);
            events::debug_event::handle(state, payload.data).await;
            StatusCode::OK
        }
    }
}