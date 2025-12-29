use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ChatUpdateData {

    /// The JID of the remote party, ex: "5216622001122@s.whatsapp.net"
    #[serde(rename = "remoteJid")]
    pub remote_jid: String,

    /// Transporter (internal) instance ID. Warning: not the same as "instance" field
    #[serde(rename = "instanceId")]
    pub instance_id: String, // Ex: "484f2677-cb3f-46fa-b67f-177a9bbb51fd",

    // ALUCINACIONES
    // pub id: String, // jid del chat
    // #[serde(rename = "unreadCount")]
    // pub unread_count: Option<u32>,
    // pub archived: Option<bool>,
    // pub pinned: Option<bool>,
    // pub name: Option<String>,
    // #[serde(rename = "conversationTimestamp")]
    // pub conversation_timestamp: Option<i64>,
}