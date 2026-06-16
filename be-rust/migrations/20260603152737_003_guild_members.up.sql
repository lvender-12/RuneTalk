CREATE TABLE guild_members (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    guild_id UUID NOT NULL REFERENCES guilds(id) ON DELETE CASCADE,
    adventurer_id UUID NOT NULL REFERENCES adventurers(id) ON DELETE CASCADE,
    nickname VARCHAR(100),
    role VARCHAR(20) NOT NULL DEFAULT 'member',
    joined_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE(guild_id, adventurer_id),
    CHECK (role IN ('owner', 'admin', 'member'))
);
