use serde::Deserialize;

// Nada que ver con el JSON indicado en la respuesta de Evolution, pero sugerido por IA
// se tiene que depurar al 100%
#[derive(Debug, Deserialize)]
pub struct ChatUpdateData {
    pub id: String, // jid del chat

    #[serde(rename = "unreadCount")]
    pub unread_count: Option<u32>,

    pub archived: Option<bool>,
    pub pinned: Option<bool>,

    pub name: Option<String>,

    #[serde(rename = "conversationTimestamp")]
    pub conversation_timestamp: Option<i64>,
}