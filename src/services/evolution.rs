/**
 * Servicio para interactuar con la API de Evolution.
 *   Aqui se implementan las llamadas HTTP necesarias para enviar mensajes.
 * 
 *                                          (c) Dic 2025  -  Aaron
 */

use reqwest::Client;
use serde::Serialize;
use tokio::time::{sleep, Duration};
use tracing::{warn, error};

use crate::config::Config;
//use std::time::Duration;

#[derive(Clone)]
pub struct EvolutionService {
    client: Client,
    base_url: String,
    api_key: String,
}

impl EvolutionService {
    pub fn new(config: &Config) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
            .expect("No se pudo crear cliente HTTP");

        Self {
            client,
            base_url: config.evolution_base_url.clone(),
            api_key: config.evolution_api_key.clone(),
        }
    }

    pub async fn send_message(&self, jid: &str, text: &str, ) -> Result<(), reqwest::Error> {
        let url = format!("{}/message/sendText", self.base_url);

        let body = SendTextRequest {
            number: jid,
            text,
        };

        //let result = 
        self.client
            .post(&url)
            .header("apikey", &self.api_key)
            .json(&body)
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }
}

#[derive(Serialize)]
struct SendTextRequest<'a> {
    number: &'a str,
    text: &'a str,
}