use sqlx::PgPool;

use crate::models::domain::incoming_message::IncomingMessage;
use crate::models::stored_messages::StoredMessage;

//use crate::db::repositories::repository_error::RepositoryError;

#[derive(Clone)]
pub struct MessageRepository {
    pool: PgPool,
}

impl MessageRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn insert_incoming( &self, message: &IncomingMessage, instance: &str ) 
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
            instance,
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


    pub async fn get_conversation_history(
        &self,
        instance: &str,
        remote_jid: &str,
        limit: i64,
    ) -> Result<Vec<StoredMessage>, sqlx::Error> {
        let messages = sqlx::query_as::<_, StoredMessage>(
            r#"
            SELECT
                id,
                evolution_id,
                instance,
                remote_jid,
                remote_jid_alt,
                text,
                timestamp,
                created_at
            FROM incoming_messages
            WHERE instance = $1
              AND remote_jid = $2
            ORDER BY created_at ASC
            LIMIT $3
            "#
        )
        .bind(instance)
        .bind(remote_jid)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(messages)
    }
}