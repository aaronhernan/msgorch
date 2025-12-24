use sqlx::PgPool;

use crate::models::domain::incoming_message::IncomingMessage;
//use crate::db::repositories::repository_error::RepositoryError;

#[derive(Clone)]
pub struct MessageRepository {
    pool: PgPool,
}

impl MessageRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn insert_incoming( &self, message: &IncomingMessage, ) 
    //-> Result<i64, RepositoryError> 
    -> Result<i64, sqlx::Error>
    {
        let record = sqlx::query!(
            r#"
            INSERT INTO incoming_messages (
                instance,
                evolution_message_id,
                remote_jid,
                remote_jid_alt,
                text,
                timestamp
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id
            "#,
            message.instance,
            message.id,
            message.remote_jid,
            message.remote_jid_alt,
            message.text,
            message.timestamp
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(record.id)
    }
}