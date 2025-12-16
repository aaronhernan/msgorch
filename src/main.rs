use axum::{
    routing::post,
    Router,
    extract::Json,
};
use serde_json::Value;
use std::fs::OpenOptions;
use std::io::Write;

async fn webhook_handler(Json(payload): Json<Value>) {
    let pretty = serde_json::to_string_pretty(&payload).unwrap();

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("messages.log")
        .expect("No se pudo abrir el archivo");

    writeln!(file, "==============================").unwrap();
    writeln!(file, "{}", pretty).unwrap();

    println!("Mensaje recibido y guardado");
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/webhook", post(webhook_handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .expect("No se pudo abrir el puerto");

    println!("Escuchando en http://127.0.0.1:3001");

    axum::serve(listener, app).await.unwrap();
}