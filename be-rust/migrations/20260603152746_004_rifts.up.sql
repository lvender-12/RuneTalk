CREATE TABLE rifts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    guild_id UUID NOT NULL REFERENCES guilds(id) ON DELETE CASCADE,
    name VARCHAR(100) NOT NULL,
    topic TEXT,
    type VARCHAR(20) NOT NULL DEFAULT 'text', -- text, voice, announcement
    position INTEGER NOT NULL DEFAULT 0,
    is_private BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE (guild_id, name),
    CHECK (type IN ('text', 'voice', 'announcement'))
);
