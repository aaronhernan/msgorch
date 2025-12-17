use reqwest::Client;
use serde::Serialize;

use crate::config::Config;

#[derive(Clone)]
pub struct EvolutionService {
    client: Client,
    base_url: String,
    api_key: String,
}

impl EvolutionService {
    pub fn new(config: &Config) -> Self {
        Self {
            client: Client::new(),
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

        self.client
            .post(url)
            .header("apikey", &self.api_key)
            .json(&body)
            .send()
            .await?
            .error_for_status()?; // 👈 HTTP 4xx/5xx = error

        Ok(())
    }
}

#[derive(Serialize)]
struct SendTextRequest<'a> {
    number: &'a str,
    text: &'a str,
}