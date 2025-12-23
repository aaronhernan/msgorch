CREATE TABLE messages (
    id TEXT PRIMARY KEY,
    instance TEXT NOT NULL,
    remote_jid TEXT NOT NULL,
    remote_jid_alt TEXT,
    text TEXT,
    timestamp BIGINT,
    from_me BOOLEAN NOT NULL,

    status TEXT NOT NULL DEFAULT 'received',

    received_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    processed_at TIMESTAMPTZ
);

CREATE INDEX idx_messages_remote_jid ON messages(remote_jid);
CREATE INDEX idx_messages_received_at ON messages(received_at);