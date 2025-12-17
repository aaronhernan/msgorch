use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub evolution_base_url: String,
    pub evolution_api_key: String,
    pub listen_host: String,
    pub listen_port: u16,
    pub webhook_token: String,
}

impl Config {
    pub fn from_env() -> Self {
        let evolution_base_url =
            env::var("EVOLUTION_BASE_URL")
                .expect("EVOLUTION_BASE_URL no está definido");

        let evolution_api_key =
            env::var("EVOLUTION_API_KEY")
                .expect("EVOLUTION_API_KEY no está definido");
        let listen_host =
            env::var("LISTEN_HOST")
                .unwrap_or_else(|_| "127.0.0.1".to_string());

        let listen_port =
            env::var("LISTEN_PORT")
                .unwrap_or_else(|_| "3001".to_string())
                .parse::<u16>()
                .expect("LISTEN_PORT debe ser un número");
        let webhook_token =
            env::var("WEBHOOK_TOKEN")
            .expect("WEBHOOK_TOKEN no está definido");

        Self {
            evolution_base_url,
            evolution_api_key,
            listen_host,
            listen_port,
            webhook_token,
        }
    }
}