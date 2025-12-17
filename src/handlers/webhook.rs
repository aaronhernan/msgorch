use axum::extract::{Json, State};
use tracing::{info, warn, error};
// use std::fs::OpenOptions;
// use std::io::Write;

use crate::{
    models::evolution::WebhookEvent,
    services::evolution::EvolutionService,
    config::Config,
};

pub async fn webhook_handler(
    State((config, evolution)): State<(Config, EvolutionService)>,
    Json(payload): Json<WebhookEvent>
) {
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

    info!(
        jid = %jid,
        text = %text,
        evolution_url = %config.evolution_base_url,
        "Mensaje entrante"
    );
    

    // Background task to send the reply
    let evolution = evolution.clone();
    tokio::spawn(async move {
        if let Err(err) = evolution.send_message(&jid, "Mensaje recibido").await {
            error!("Error enviando mensaje: {}", err);
        }
    });

    // Terminamos execución del handler, y respondemos 200 OK inmediatamente
    
    // let mut file = OpenOptions::new()
    //     .create(true)
    //     .append(true)
    //     .open("messages.log")
    //     .expect("No se pudo abrir messages.log");
    // writeln!(file, "{} | {}", jid, text).unwrap();
}