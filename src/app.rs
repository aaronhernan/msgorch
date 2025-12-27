use axum::{Router,};
use deadpool_redis::{Config as RedisConfig, Runtime};
use tokio::net::TcpListener;
use tracing::info;

use crate::{
    config::Config, 
    db::repositories::messages::MessageRepository,
    db::pool::create_pool,
    handlers,
    idempotency::RedisIdempotencyStore,
    middleware, services::evolution::EvolutionService
};

// pub type AppState = (
//     Config, 
//     EvolutionService,
//     RedisIdempotencyStore,
// );
#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub evolution: EvolutionService,
    pub idempotency: RedisIdempotencyStore,
    pub message_repository: MessageRepository,
}

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route(
            "/webhook",
            axum::routing::post(handlers::webhook::webhook_handler),
        )
        .route_layer(axum::middleware::from_fn_with_state(
            state.clone(),
            middleware::webhook_auth,
        ))
        .with_state(state)
}

//pub async fn run(config: Config) -> Result<(), std::io::Error> {
pub async fn run(config: Config) -> anyhow::Result<()> {
    let addr = format!("{}:{}", config.listen_host, config.listen_port);
    let listener = TcpListener::bind(addr).await?;

    run_with_listener(listener, config).await
}

//pub async fn run_with_listener( listener: TcpListener, config: Config,) -> Result<(), std::io::Error> {
pub async fn run_with_listener( listener: TcpListener, config: Config,) -> anyhow::Result<()> {
    // Creacion del pool de conexiones a Redis
    let redis_cfg = RedisConfig::from_url(&config.redis_url);
    //let mut redis_cfg = RedisConfig::from_url(&config.redis_url);
    let redis_pool = redis_cfg.create_pool(Some(Runtime::Tokio1))?;

    // Configuracion del store de idempotencia
    let idempotency = RedisIdempotencyStore::new(
        redis_pool,
        config.redis_prefix.clone(),
        config.idempotency_ttl,
    );

    let evolution = EvolutionService::new(&config);

    //let message_repository = MessageRepository::new(sqlx::PgPool::connect(&config.database_url).await?);
    let pg_pool = create_pool(&config.database_url).await?;
    let message_repository = MessageRepository::new(pg_pool.clone());
    //let state = (config.clone(), evolution, idempotency);
    // Comentario de aprendizaje. Vamos a meter el ownership de los elementos en AppState
    //   el state es el dueno de sus dependencias
    let state = AppState { config, evolution, idempotency, message_repository, };
    let app = build_router(state);

    //let current_filter = tracing::metadata::LevelFilter::current();
    //print!("Current filter: {:?}", current_filter);

    info!("Escuchando en http://{}", listener.local_addr()?);

    axum::serve(listener, app).await?;
    Ok(())
}
