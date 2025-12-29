use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ApiMessage {
    /// ID del mensaje
    pub id: String,
    /// Texto del mensaje
    pub text: String,
    /// JID del destinatario
    pub destination_jid: String,
    /// Timestamp original del mensaje (si existe)
    pub timestamp: Option<i64>,
}
