CREATE TABLE messages (
    id BIGSERIAL PRIMARY KEY,
    from_me BOOLEAN NOT NULL,
    instance TEXT NOT NULL,
    transporter_message_id TEXT,
    remote_jid TEXT NOT NULL,
    remote_jid_alt TEXT,
    text TEXT NOT NULL,
    timestamp BIGINT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT uq_evolution_message UNIQUE (instance, transporter_message_id)
);

-- Mensajes por conversación
CREATE INDEX idx_messages_remote_jid_created_at
ON messages (remote_jid, instance, created_at DESC);

-- Indice para consultas por remote_jid
-- No se estaria utilizando, (creo), por que siempre tengo que agregar la instancia
-- CREATE INDEX idx_messages_remote_jid
-- ON messages (remote_jid);

-- Mensajes por instancia
-- No lo voy a agregar, por que al crear la restriccion UNIQUE en (instance, transporter_message_id)
-- ya se crea un indice unico que sirve para este proposito
-- CREATE INDEX idx_messages_instance_created_at
-- ON messages (instance, created_at DESC);

-- Indice para consultas por fecha
-- CREATE INDEX idx_messages_created_at
-- ON messages (created_at);
-- Lookup rápido por evolution_id
-- CREATE UNIQUE INDEX idx_messages_evolution_id ON incoming_messages (evolution_id) WHERE evolution_id IS NOT NULL;