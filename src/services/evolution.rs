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

    pub async fn send_message(
        &self,
        jid: &str,
        text: &str,
    ) -> Result<(), reqwest::Error> {
        let url = format!("{}/message/sendText", self.base_url);

        let body = SendTextRequest {
            number: jid,
            text,
        };

        let mut attempt = 0;

        loop {
            attempt += 1;

            let result = self.client
                .post(&url)
                .header("apikey", &self.api_key)
                .json(&body)
                .send()
                .await;
            match result {
                Ok(resp) => {
                    if resp.status().is_success() {
                        return Ok(());
                    }

                    if resp.status().as_u16() >= 500 && attempt < 3 {
                        warn!(
                            attempt,
                            status = %resp.status(),
                            "Error 5xx, reintentando"
                        );
                    } else {
                        error!(
                            status = %resp.status(),
                            "Error HTTP no recuperable"
                        );
                        return Err(resp.error_for_status().unwrap_err());
                    }
                }

                Err(err) => {
                    if attempt < 3 {
                        warn!(
                            attempt,
                            error = %err,
                            "Error de red, reintentando"
                        );
                    } else {
                        error!("Error definitivo enviando mensaje: {}", err);
                        return Err(err);
                    }
                }
            }
            let backoff = Duration::from_millis(500 * attempt);
            sleep(backoff).await;
        }
    }
}

#[derive(Serialize)]
struct SendTextRequest<'a> {
    number: &'a str,
    text: &'a str,
}