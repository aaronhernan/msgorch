CREATE TABLE incoming_messages (
    id BIGSERIAL PRIMARY KEY,
    instance TEXT NOT NULL,
    evolution_message_id TEXT NOT NULL,
    remote_jid TEXT NOT NULL,
    remote_jid_alt TEXT,
    text TEXT,
    timestamp BIGINT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT uq_evolution_message UNIQUE (instance, evolution_message_id)
);

CREATE INDEX idx_incoming_messages_remote_jid
ON incoming_messages (remote_jid);

CREATE INDEX idx_incoming_messages_created_at
ON incoming_messages (created_at);