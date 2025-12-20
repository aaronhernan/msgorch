use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct WebhookEnvelope {
    pub event: String, // Tipos de eventos
    pub instance: String, // Nombre de la instancia, como guarda evolution
    pub data: Value, // por ahora genérico

    pub destination: Option<String>, // URL destino del webhook (this APP)
    pub date_time: Option<String>, // Fecha y hora del evento
    pub sender: Option<String>, // JID del emisor
    pub server_url: Option<String>, // URL del servidor evolution
    pub apikey: Option<String>,  // Api key de la INSTANCIA de evolution, No se para que sirve
}