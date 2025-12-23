use sqlx::PgPool;
use crate::models::domain::IncomingMessage;

pub struct MessageRepository;

impl MessageRepository {
    pub async fn insert(
        pool: &PgPool,
        instance: &str,
        msg: &IncomingMessage,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO messages (
                id,
                instance,
                remote_jid,
                remote_jid_alt,
                text,
                timestamp,
                from_me
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (id) DO NOTHING
            "#,
            msg.id,
            instance,
            msg.remote_jid,
            msg.remote_jid_alt,
            msg.text,
            msg.timestamp,
            msg.from_me,
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn mark_processed(
        pool: &PgPool,
        message_id: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE messages
            SET status = 'processed',
                processed_at = now()
            WHERE id = $1
            "#,
            message_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}