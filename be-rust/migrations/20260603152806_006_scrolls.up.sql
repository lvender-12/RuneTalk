CREATE TABLE scrolls (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    initiator_id UUID NOT NULL REFERENCES adventurers(id) ON DELETE CASCADE,
    recipient_id UUID NOT NULL REFERENCES adventurers(id) ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    CHECK (initiator_id != recipient_id)
);

CREATE UNIQUE INDEX idx_scrolls_pair ON scrolls (
    LEAST(initiator_id, recipient_id),
    GREATEST(initiator_id, recipient_id)
);
