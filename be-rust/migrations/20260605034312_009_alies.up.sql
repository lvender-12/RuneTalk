CREATE TABLE allies (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    id_1 UUID NOT NULL REFERENCES adventurers(id) ON DELETE CASCADE,
    id_2 UUID NOT NULL REFERENCES adventurers(id) ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    CHECK (id_1 != id_2)
);

CREATE UNIQUE INDEX idx_allies_pair ON allies (
    LEAST(id_1, id_2),
    GREATEST(id_1, id_2)
);
