use axum::http::StatusCode;
use serde_json::Value;
use tracing::{error, warn};
use crate::{
    app::AppState, events::message_processor::process_message, models::{domain::incoming_message::IncomingMessage, evolution::message_upsert::MessageUpsertData}
};

fn map_to_domain(parsed: MessageUpsertData) -> IncomingMessage {
    IncomingMessage {
        id: parsed.key.id.clone(),
        //instance: parsed.instance.clone(),
        instance: "servicewa".to_string(),
        remote_jid: parsed.key.remote_jid.clone(),
        remote_jid_alt: parsed.key.remote_jid_alt.clone(),
        text: parsed.message.conversation.clone(),
        from_me: parsed.key.from_me,
        timestamp: parsed.message_timestamp,
    }
}

fn validate_message(message: &IncomingMessage) -> bool {
    // Validaciones basicas
    if message.id.is_empty() || message.remote_jid.is_empty() {
        return false;
    }
    true
}

pub async fn handle(state: &AppState, data: Value,) -> StatusCode {
    let parsed: MessageUpsertData = match serde_json::from_value(data) {
        Ok(v) => v,
        Err(err) => {
            error!("Payload con capacidades diferentes: {}", err);
            return StatusCode::OK;
        }
    };
    
    let message = map_to_domain(parsed);
    
    if !validate_message(&message){
        warn!(message_id = %message.id, "Mensaje con datos invalidos");
        return StatusCode::OK;
    }

    match state.idempotency.check_and_mark(&message.id).await {
        Ok(false) => {
            warn!(message_id = %message.id, "Mensaje duplicado ignorado");
            return StatusCode::OK;
        }
        Err(err) => {
            error!(err = %err, "Error en idempotencia");
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
        _ => {}
    }

    if let Err(err) = process_message(state, message).await {
        error!(err = %err, "Error procesando mensaje");
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    // El procesamiento, logging y respuesta asincrona lo va a hacer message_processor.rs
    StatusCode::OK
}