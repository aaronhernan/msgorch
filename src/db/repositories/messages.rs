use sqlx::PgPool;

use crate::models::message::Message;
//use crate::db::repositories::repository_error::RepositoryError;

#[derive(Clone)]
pub struct MessageRepository {
    pool: PgPool,
}

impl MessageRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn insert_incoming( &self, message: &Message, instance: &str ) 
    //-> Result<i64, RepositoryError> 
    -> Result<i64, sqlx::Error>
    {
        let record = sqlx::query!(
            r#"
            INSERT INTO messages (
                from_me,
                instance,
                transporter_message_id,
                remote_jid,
                remote_jid_alt,
                text,
                timestamp
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id
            "#,
            message.from_me,
            instance,
            message.transporter_message_id,
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
    ) -> Result<Vec<Message>, sqlx::Error> {
        let messages = sqlx::query_as::<_, Message>(
            r#"
            SELECT
                id,
                from_me,
                instance,
                transporter_message_id,
                remote_jid,
                remote_jid_alt,
                text,
                timestamp,
                created_at
            FROM messages
            WHERE instance = $1
              AND remote_jid = $2
            ORDER BY created_at ASC
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