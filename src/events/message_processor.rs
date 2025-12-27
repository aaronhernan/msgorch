use tracing::{debug, error, info};
use crate::{app::AppState, models::message::Message};
use std::{fmt, time::Duration};
use rand::Rng;

const MAX_RETRIES: u8 = 5;
const BASE_DELAY_MS: u64 = 1000;

#[derive(Debug)]
pub enum ProcessError {
    EvolutionError(String),
    Retryable(String),
    Fatal(String),
}

impl ProcessError {
    fn is_retryable(&self) -> bool {
        matches!(self, ProcessError::Retryable(_) | ProcessError::EvolutionError(_))
    }
}

impl fmt::Display for ProcessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProcessError::EvolutionError(err) => write!(f, "error enviando mensaje a Evolution: {}", err),
            ProcessError::Retryable(err) => write!(f, "error retryable: {}", err),
            ProcessError::Fatal(err) => write!(f, "error fatal: {}", err),
        }
    }
}

pub async fn process_message(
    state: &AppState,
    message: Message,
    instance: &str,
) -> Result<(), ProcessError> {

    
    // Filtros
    if message.from_me {
        tracing::debug!( transporter_id = %message.transporter_id, remote_jid = %message.remote_jid, "Mensaje ignorado (from_me)" );
        return Ok(());
    }

    let max_attempts = MAX_RETRIES;
    let base_delay_ms = BASE_DELAY_MS;
    let mut attempt = 0;
    loop {
        attempt += 1;
        match handle_message(state, &message, instance).await {
            Ok(_) => {
                //info!( transporter_id = %message.transporter_id, remote_jid = %message.remote_jid, "Mensaje procesado correctamente" );
                debug!( transporter_id = %message.transporter_id, remote_jid = %message.remote_jid, text = %message.text, "Procesamiento exitoso" );
                return Ok(());
            }
            Err(err) => {
                if !err.is_retryable() {
                    error!( transporter_id = %message.transporter_id, remote_jid = %message.remote_jid, error = %err, "Error permanente, no se reintenta" );
                    return Err(err);
                }
                // Aqui se supone que es retryable, vemos si agotamos reintentos
                if attempt >= max_attempts {
                    error!( transporter_id = %message.transporter_id, remote_jid = %message.remote_jid, error = %err, "Se agotaron los reintentos" );
                    return Err(err);
                }
                // Aqui es donde reintentamos
                // Backoff exponencial con jitter
                let max_delay = base_delay_ms * (1 << (attempt - 1));
                let jitter: u64 = rand::rng().random_range(0..=max_delay);
                debug!( transporter_id = %message.transporter_id, remote_jid = %message.remote_jid, error = %err, delay_ms = jitter, "Reintentando con backoff" );
                tokio::time::sleep(Duration::from_millis(jitter)).await;
            }
        }
    }
    // El resto del procesamiento lo hace handle_message
    // Logging
    //info!(transporter_id = %message.id,jid = %message.remote_jid, texto = %text, "Mensaje entrante" );

    // Acciones
    // state
    //     .evolution
    //     .send_message(&message.remote_jid, "Mensaje recibido")
    //     .await?;

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
    // Ok(())
}

async fn handle_message(
    state: &AppState,
    message: &Message,
    instance: &str,
) -> Result<(), ProcessError> {
    //let text = message.text.clone();
    
    let db_id = state
    .message_repository
    .insert_incoming(&message, instance)
    .await;
    info!("Mensaje insertado con ID: {:?}", db_id);
    //info!( transporter_id = %message.transporter_id, remote_jid = %message.remote_jid, texto = %text, "Mensaje entrante" );

    // Ahora aqui es donde decidimos:
    // reglas
    // workflows
    // IA
    // colas

    // Contestar o no contestar...
    // state.evolution.send_message(&message.remote_jid, "Mensaje recibido")
    //     .await.map_err(|err| {
    //         ProcessError::Retryable(format!("Error enviando mensaje a Evolution API: {err}"))
    //     })?;
    Ok(())
}
