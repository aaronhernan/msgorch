use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Message {
    /// ID interno (PK)
    pub id: Option<i64>,

    /// ID original de Evolution / WhatsApp
    pub evolution_id: String,

    /// Instancia de Evolution (WhatsApp)
    pub instance: String,

    /// JID remoto principal
    pub remote_jid: String,

    /// JID alternativo (fallback / debug)
    pub remote_jid_alt: Option<String>,

    /// Texto del mensaje
    pub text: String,

    /// Timestamp original del mensaje (si existe)
    pub timestamp: Option<i64>,

    /// Cuándo lo recibimos / persistimos
    pub created_at: DateTime<Utc>,
}