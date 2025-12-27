use tracing;
use axum::http::StatusCode;

use crate::{
    app::AppState,
    models::webhook::WebhookEnvelope,
    events,
};

pub async fn dispatch( payload: WebhookEnvelope, state: &AppState, ) -> StatusCode {
    let span = tracing::info_span!(
        "webhook_event",
        app = %state.config.app_name,
        instance = %payload.instance,
        event = %payload.event,
        remote_jid = tracing::field::Empty,
        message_id = tracing::field::Empty,
    );
    let _enter = span.enter();

    match payload.event.as_str() {
        "messages.upsert"   => { events::message_upsert::handle(state, payload.data, &payload.instance).await }
        //"messages.update"   => { events::message_update::handle(state, payload.data).await }
        //"messages.delete"   => { events::message_delete::handle(state, payload.data).await }
        //"connection.update" => { events::connection_update::handle(state, payload.data).await }
        //"presence.update"   => { events::presence_update::handle(state, payload.data).await }
        //"chats.update"      => { events::chats_update::handle(state, payload.data).await }
        _ => {
            tracing::warn!("Evento no manejado");
            if tracing::enabled!(tracing::Level::DEBUG) {
                //debug!("Evento de depuración recibido con datos:\n{:?}", payload.data);
                tracing::debug!("Evento de depuración recibido con datos :\n{:?}", payload);
            }
            StatusCode::OK
        }
    }
}