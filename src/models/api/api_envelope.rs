use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct ApiEnvelope {
    pub event: String, // Tipos de eventos
    pub instance_id: u64, // ID de la instancia que envía
    pub instance: String, // Nombre de la instancia, o sistema que envía
    pub data: Value, // ex: model Message por lo normal
}
