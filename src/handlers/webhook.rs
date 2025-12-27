use axum::{
    extract::State, 
    http::StatusCode, 
    Json, 
    body::Bytes,
};

use crate::{
    app::AppState, 
    events::dispatcher, 
    models::webhook::WebhookEnvelope
};

// pub async fn webhook_handler(
//     State(state): State<AppState>,
//     Json(payload): Json<WebhookEnvelope>
// ) -> StatusCode {
//     dispatcher::dispatch( payload, &state).await
// }

pub async fn webhook_handler(
    State(_state): State<AppState>,
    body: Bytes,
) -> StatusCode {
    
    match std::str::from_utf8(&body) {
        Ok(text) => {
            tracing::info!("Webhook RAW body:\n{}", text);
        }
        Err(err) => {
            tracing::error!("Body no es UTF-8 válido: {}", err);
        }
    }

    StatusCode::OK
}

/*
pub async fn handler_anterior_acoplado(
    State(state): State<AppState>,
    Json(payload): Json<WebhookEnvelope>
) -> StatusCode {
    if payload.event != "messages.upsert" {
        warn!("Evento ignorado: {}", payload.event);
        return StatusCode::OK;
    }

    let Some(message_id) = payload
        .data
        .get("key")
        .and_then(|k| k.get("id"))
        .and_then(|id| id.as_str())
    else {
        return StatusCode::OK;
    };

    match state.idempotency.check_and_mark(message_id).await {
        Ok(false) => {
            info!("Mensaje duplicado ignorado: {}", message_id);
            return StatusCode::OK;
        }
        Ok(true) => {
            info!("Mensaje nuevo procesado: {}", message_id);
        }
        Err(err) => {
            error!("Error en idempotencia: {}", err);
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    }
    // Tambien podemos extraer el texo...
    let Some(text) = payload.data.message.conversation.as_deref()

    // Enviar la respuesta en background, para no bloquear el handler
    // Clonamos el servicio desde el estado, para que tokio utilice una referencia estatica
    let Some(jid) = payload
        .data
        .get("key")
        .and_then(|k| k.get("remoteJid"))
        .and_then(|remote_jid| remote_jid.as_str())
        .map(|s| s.to_string())
    else {
        error!("No se pudo extraer el JID del mensaje");
        return StatusCode::OK;
    };

    let evolution = state.evolution.clone();
    tokio::spawn(async move {
        if let Err(err) = evolution
            .send_message(&jid, "Mensaje recibido")
            .await
        {
            error!("Error enviando mensaje: {}", err);
        }
    });
    
    if let Some(text) = payload
        .data
        .get("message")
        .and_then(|m| m.get("conversation"))
    {
        //info!("Texto: {}", text);
        info!(
            //jid = %jid,
            //text = %payload.data.message.conversation.as_deref().unwrap_or("N/A"),
            texto = %text,
            //evolution_url = %config.evolution_base_url,
            "Mensaje entrante"
        );
    }

    // Terminamos execución del handler, y respondemos 200 OK inmediatamente    
    // let mut file = OpenOptions::new()
    //     .create(true)
    //     .append(true)
    //     .open("messages.log")
    //     .expect("No se pudo abrir messages.log");
    // writeln!(file, "{} | {}", jid, text).unwrap();
    StatusCode::OK
}
*/
