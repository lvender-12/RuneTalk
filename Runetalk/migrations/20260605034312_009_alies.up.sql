CREATE TABLE allies (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    from_id UUID NOT NULL REFERENCES adventurers(id) ON DELETE CASCADE,
    to_id UUID NOT NULL REFERENCES adventurers(id) ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    CHECK (from_id != to_id)
);

CREATE UNIQUE INDEX idx_allies_pair ON allies (
    LEAST(from_id, to_id),
    GREATEST(from_id, to_id)
);
