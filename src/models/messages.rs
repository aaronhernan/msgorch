use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MessageUpsertData {
    pub key: MessageKey, // Informacion principal del mensaje
    pub pushName: Option<String>, // Nombre del contacto que envio el mensaje
    pub message: MessageContent, // Contenido del mensaje
    pub messageType: Option<String>, // Tipo de mensaje (conversation, ??)
    pub messageTimestamp: Option<i64>, // Timestamp del mensaje
    pub source: Option<String>, // Fuente del mensaje (web, movil, ??)
}

#[derive(Debug, Deserialize)]
pub struct MessageKey {
    pub remoteJid: String, // JID del contacto de whatsapp
    pub remoteJidAlt: Option<String>, // JID alternativo del contacto de whatsapp
    pub fromMe: bool, // Si estoy enviando o recibiendo el mensaje
    pub id: String, // ID REAL, de whatsapp
}

#[derive(Debug, Deserialize)]
pub struct MessageContent {
    pub conversation: Option<String>,
    // pub messageContextInfo: Option<Value>, // Contexto del mensaje (dispositivo, messageSecret)
}