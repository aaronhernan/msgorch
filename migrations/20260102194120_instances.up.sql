
CREATE TABLE instances (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    jid VARCHAR(255) NOT NULL,
    alternative_jid VARCHAR(255) NOT NULL,
    token VARCHAR(255) NOT NULL,
    modality INT NOT NULL,
    -- Los creditos, son centimos (c), asi 1 = .01 creditos, y 100 = 1 credito
    credits BIGINT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
