use axum::http::StatusCode;
use serde_json::Value;
use tracing::{info, error};
use crate::{
    app::AppState, 
    models::{messages::MessageUpsertData}
};

pub async fn handle(state: &AppState, data: Value,) -> StatusCode {

    let parsed: MessageUpsertData = match serde_json::from_value(data) {
        Ok(v) => v,
        Err(err) => {
            error!("Error parseando messages.upsert: {}", err);
            return StatusCode::OK;
        }
    };

    // Extraer el ID del mensaje y el JID remoto, igual pero de forma distinta
    let message_id = parsed.key.id.as_str();
    let jid = parsed.key.remote_jid.clone();

    match state.idempotency.check_and_mark(message_id).await {
        Ok(false) => {
            info!("Mensaje duplicado ignorado: {}", message_id);
            return StatusCode::OK;
        }
        Err(err) => {
            error!("Error en idempotencia: {}", err);
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
        _ => {}
    }

    let texto = parsed
        .message
        .conversation
        .as_deref()
        .unwrap_or("<sin texto>");
    info!(jid = %jid, text = %texto, "Mensaje entrante");

    // let evolution = state.evolution.clone();

    // tokio::spawn(async move {
    //     if let Err(err) = evolution
    //         .send_message(&jid, "Mensaje recibido")
    //         .await
    //     {
    //         error!("Error enviando mensaje: {}", err);
    //     }
    // });    

    StatusCode::OK
}