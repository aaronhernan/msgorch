use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MessageUpsertData {
    pub key: MessageKey, // Informacion principal del mensaje
    #[serde(rename = "pushName")]
    pub push_name: Option<String>, // Nombre del contacto que envio el mensaje
    pub message: MessageContent, // Contenido del mensaje
    #[serde(rename = "messageType")]
    pub message_type: Option<String>, // Tipo de mensaje (conversation, ??)
    #[serde(rename = "messageTimestamp")]
    pub message_timestamp: Option<i64>, // Timestamp del mensaje
    pub source: Option<String>, // Fuente del mensaje (web, movil, ??)
}

#[derive(Debug, Deserialize)]
pub struct MessageKey {
    #[serde(rename = "remoteJid")]
    pub remote_jid: String, // JID del contacto de whatsapp
    #[serde(rename = "remoteJidAlt")]
    pub remote_jid_alt: Option<String>, // JID alternativo del contacto de whatsapp
    #[serde(rename = "fromMe")]
    pub from_me: bool, // Si estoy enviando o recibiendo el mensaje
    pub id: String, // ID REAL, de whatsapp
}

#[derive(Debug, Deserialize)]
pub struct MessageContent {
    pub conversation: Option<String>,
    // pub messageContextInfo: Option<Value>, // Contexto del mensaje (dispositivo, messageSecret)
}