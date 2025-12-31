use crate::{
    app::AppState, events::message_processor::process_message,
    models::evolution::message_upsert::MessageUpsertData, models::message::Message,
};
use axum::http::StatusCode;
use chrono::DateTime;
use serde_json::Value;
use tracing::{error, warn};

fn map_to_domain(parsed: MessageUpsertData, instance: &str) -> Message {
    Message {
        id: None,
        instance: instance.to_string().clone(),
        transporter_message_id: parsed.key.id.clone(),
        remote_jid: parsed.key.remote_jid.clone(),
        remote_jid_alt: parsed.key.remote_jid_alt.clone(),
        text: parsed.message.conversation.clone(),
        from_me: parsed.key.from_me,
        origin_timestamp: DateTime::from_timestamp(parsed.message_timestamp.unwrap(), 0),
        created_at: chrono::Utc::now(),
    }
}

fn validate_message(message: &Message) -> bool {
    // Validaciones basicas, casi ejemplo, solo para dejar el lugar donde agregar mas
    if message.remote_jid.is_empty() {
        return false;
    }
    true
}

pub async fn handle(state: &AppState, data: Value, instance: &str) -> StatusCode {
    let parsed: MessageUpsertData = match serde_json::from_value(data) {
        Ok(v) => v,
        Err(err) => {
            error!("Payload con capacidades diferentes: {}", err);
            return StatusCode::OK;
        }
    };

    let message = map_to_domain(parsed, instance);

    if !validate_message(&message) {
        warn!(transporter_message_id = %message.transporter_message_id, "Mensaje con datos invalidos");
        return StatusCode::OK;
    }

    match state
        .idempotency
        .check_and_mark(&message.transporter_message_id)
        .await
    {
        Ok(false) => {
            warn!(transporter_message_id = %message.transporter_message_id, "Mensaje duplicado ignorado");
            return StatusCode::OK;
        }
        Err(err) => {
            error!(err = %err, "Error en idempotencia");
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
        _ => {}
    }

    if let Err(err) = process_message(state, message, instance).await {
        error!(err = %err, "Error procesando mensaje");
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    // El procesamiento, logging y respuesta asincrona lo va a hacer message_processor.rs
    StatusCode::OK
}
