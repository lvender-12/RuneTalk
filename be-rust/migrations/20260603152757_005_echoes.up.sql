CREATE TABLE echoes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    rift_id UUID NOT NULL REFERENCES rifts(id) ON DELETE CASCADE,
    adventurer_id UUID NOT NULL REFERENCES adventurers(id) ON DELETE CASCADE,
    reply_to_id UUID REFERENCES echoes(id) ON DELETE SET NULL,
    content TEXT NOT NULL,
    message_type VARCHAR(20) NOT NULL DEFAULT 'text', -- text, image, file
    is_pinned BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    edited_at TIMESTAMP,
    CHECK (message_type IN ('text', 'image', 'file'))
);

CREATE OR REPLACE FUNCTION check_echo_reply_same_rift()
RETURNS TRIGGER AS $$
DECLARE
    parent_rift UUID;
BEGIN
    IF NEW.reply_to_id IS NULL THEN
        RETURN NEW;
    END IF;

    SELECT rift_id INTO parent_rift FROM echoes WHERE id = NEW.reply_to_id;
    IF parent_rift IS NULL THEN
        RAISE EXCEPTION 'reply target echo not found';
    END IF;
    IF parent_rift IS DISTINCT FROM NEW.rift_id THEN
        RAISE EXCEPTION 'reply must be in the same rift';
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_echoes_reply_same_rift
    BEFORE INSERT OR UPDATE OF reply_to_id, rift_id ON echoes
    FOR EACH ROW
    EXECUTE FUNCTION check_echo_reply_same_rift();
