use axum::{
    routing::post,
    Router,
};
use tokio::net::TcpListener;
use tracing::info;

use crate::handlers::webhook::webhook_handler;

pub async fn run() {
    let app = Router::new()
        .route("/webhook", post(webhook_handler));

    let listener = TcpListener::bind("127.0.0.1:3001")
        .await
        .expect("No se pudo abrir el puerto");

    info!("Escuchando en http://127.0.0.1:3001");

    axum::serve(listener, app)
        .await
        .unwrap();
}
