CREATE TABLE pledges (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    from_id UUID NOT NULL REFERENCES adventurers(id) ON DELETE CASCADE,
    to_id UUID NOT NULL REFERENCES adventurers(id) ON DELETE CASCADE,
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    CHECK (from_id != to_id),
    CHECK (status IN ('pending', 'accepted', 'rejected', 'blocked')),
    UNIQUE (from_id, to_id)
);
