use std::env;

use axum::{
    Json, extract::State, http::StatusCode
};
use tracing::error;

use crate::{
    app::AppState, 
    models::{
        api::api_envelope::ApiEnvelope,
        api::api_message::ApiMessage, 
        message::Message
    },
};

fn map_to_domain(evelope: ApiEnvelope) -> Result<Message, serde_json::Error> {
    let api_message: ApiMessage = serde_json::from_value(evelope.data)?;
    // {
    //     Ok(v) => v,
    //     Err(err) => { return Err(err); }
    // };
    Ok(Message {
        id: None,
        instance: evelope.instance.clone(),
        transporter_message_id: api_message.id.clone(),
        remote_jid: api_message.destination_jid.clone(),
        remote_jid_alt: None,
        text: api_message.text.clone(),
        from_me: true,
        timestamp: api_message.timestamp.clone(),
        created_at: chrono::Utc::now(),
    })
}

pub async fn message_handler(
    State(_state): State<AppState>,
    Json(payload): Json<ApiEnvelope>
) -> StatusCode {
    let message = match map_to_domain(payload)
    {
        Ok(msg) => msg,
        Err(err) => {
            error!("Error al mapear el mensaje: {}", err);
            return StatusCode::BAD_REQUEST;
        }
    };
    tracing::info!("Mensaje recibido: {:?}", message);
    StatusCode::OK
}