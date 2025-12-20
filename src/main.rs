use msgorch::{app, config::Config};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Cargar variables de entorno desde .env si existen
    dotenvy::dotenv().ok();
    // Cargar configurar desde variables de entorno, las cuales pudieron haber provenido de .env
    let config = Config::from_env();
    tracing_subscriber::fmt::init();
    //app::run(config).await?;
    if let Err(err) = app::run(config).await {
        tracing::error!(error = %err, "La aplicación terminó con error");
        std::process::exit(1);
    }
    Ok(())
}
