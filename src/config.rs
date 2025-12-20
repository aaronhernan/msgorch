use std::env;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct Config {
    pub listen_host: String,
    pub listen_port: u16,
    pub webhook_token: String,

    pub evolution_base_url: String,
    pub evolution_api_key: String,

    pub redis_url: String,
    pub redis_prefix: String,
    pub idempotency_ttl: Duration,
}

impl Config {
    pub fn from_env() -> Self {
        let idempotency_ttl_secs: u64 = std::env::var("IDEMPOTENCY_TTL_SECS")
            .unwrap_or_else(|_| "300".into())
            .parse()
            .expect("IDEMPOTENCY_TTL_SECS inválido");
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
        let redis_url =
            env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
        let redis_prefix =
            env::var("REDIS_PREFIX")
                .unwrap_or_else(|_| "msgorch:idempotency".to_string());
        Self {
            evolution_base_url,
            evolution_api_key,
            listen_host,
            listen_port,
            webhook_token,
            redis_url,
            redis_prefix,
            idempotency_ttl: Duration::from_secs(idempotency_ttl_secs),
        }
    }
}