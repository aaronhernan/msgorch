#[derive(Debug)]
pub enum EventKind {
    MessagesUpsert,
    ConnectionUpdate,
    ContactsUpdate,
    ChatsUpdate,
    Unknown(String),
}

impl EventKind {
    pub fn from_str(event: &str) -> Self {
        match event {
            "messages.upsert" => Self::MessagesUpsert,
            "connection.update" => Self::ConnectionUpdate,
            "contacts.update" => Self::ContactsUpdate,
            "chats.update" => Self::ChatsUpdate,
            other => Self::Unknown(other.to_string()),
        }
    }
}