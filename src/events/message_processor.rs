use tracing::info;

use crate::{app, models::domain::incoming_message::IncomingMessage};

pub async fn process_message(
    state: &app::AppState,
    message: IncomingMessage,
) -> Result<(), Box<dyn std::error::Error>> {

    // Filtros
    if message.from_me {
        return Ok(());
    }
    let Some(text) = &message.text else {
        return Ok(());
    };

    // Logging
    info!(message_id = %message.id,jid = %message.remote_jid, texto = %text, "Mensaje entrante" );

    // Acciones
    state
        .evolution
        .send_message(&message.remote_jid, "Mensaje recibido")
        .await?;

    // Accion asincrona, pero dejamos de lado el manero de errores en caso de que falle:
    // let evolution = state.evolution.clone();
    // tokio::spawn(async move {
    //     if let Err(err) = evolution
    //         .send_message(&message.remote_jid, "Mensaje recibido")
    //         .await
    //     {
    //         error!("Error enviando mensaje: {}", err);
    //     }
    // });

    // Aquí va:
    // - IA
    // - reglas
    // - flujos
    // - storage
    // - respuestas
    Ok(())
}
