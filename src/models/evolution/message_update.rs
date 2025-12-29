use serde::Deserialize;

// Estructura generica, por si fallan las versiones
// #[derive(Debug, Deserialize)]
// pub struct MessageUpdateData {
//     pub key: MessageKey,
//     pub update: serde_json::Value,
// }

#[derive(Debug, Deserialize)]
pub struct MessageUpdateData {

    /// Equal to transporter_id, id coming from the transporter instance
    #[serde(rename = "keyId")]
    pub key_id: String, // Ex: "3EB0847B419598C9293971",

    /// The JID of the remote party, ex: "5216622001122@s.whatsapp.net"
    #[serde(rename = "remoteJid")]
    pub remote_jid: String,

    /// Boolean indicating if the message is from me
    #[serde(rename = "fromMe")]
    pub from_me: bool, // true, false

    /// Status of the message update, ex: "sent", "delivered", "read"
    pub status: Option<String>, // sent, delivered, read, etc.

    /// Transporter (internal) instance ID. Warning: not the same as "instance" field
    #[serde(rename = "instanceId")]
    pub instance_id: String, // Ex: "484f2677-cb3f-46fa-b67f-177a9bbb51fd",

    /// Message ID, Warning: not the same as "keyId" or "transporter_id"
    #[serde(rename = "messageId")]
    pub message_id: String, // Ex: "cmjnx17pb0027sy6cezky6jhv"
}