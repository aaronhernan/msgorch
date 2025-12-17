mod app;
mod handlers;
mod models;
mod config;
mod services;
mod middleware;

use config::Config;

#[tokio::main]
async fn main() {
    // Cargar variables de entorno desde .env si existen
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt::init();
    // Cargar configurar desde variables de entorno, las cuales pudieron haber provenido de .env
    let config = Config::from_env();
    app::run(config).await;

}
