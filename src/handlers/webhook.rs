use axum::extract::Json;
use std::fs::OpenOptions;
use std::io::Write;
use tracing::{info, warn};

use crate::models::evolution::WebhookEvent;

pub async fn webhook_handler(Json(payload): Json<WebhookEvent>) {
    if payload.event != "MESSAGES_UPSERT" {
        warn!("Evento ignorado: {}", payload.event);
        return;
    }

    if payload.data.key.from_me {
        warn!("Mensaje propio ignorado");
        return;
    }

    let text = match payload
        .data
        .message
        .and_then(|m| m.conversation)
    {
        Some(t) => t,
        None => {
            warn!("Mensaje sin texto");
            return;
        }
    };

    let jid = payload.data.key.remote_jid;

    info!(jid = %jid, text = %text, "Mensaje entrante");

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("messages.log")
        .expect("No se pudo abrir messages.log");

    writeln!(file, "{} | {}", jid, text).unwrap();
}