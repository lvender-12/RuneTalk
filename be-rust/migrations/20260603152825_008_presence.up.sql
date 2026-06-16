CREATE TABLE presence (
    adventurer_id UUID PRIMARY KEY REFERENCES adventurers(id) ON DELETE CASCADE,
    status VARCHAR(20) NOT NULL DEFAULT 'offline', -- online, idle, dnd, offline
    custom_status TEXT,
    last_seen TIMESTAMP NOT NULL DEFAULT NOW(),
    CHECK (status IN ('online', 'idle', 'dnd', 'offline'))
);
