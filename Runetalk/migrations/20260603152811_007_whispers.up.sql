CREATE TABLE whispers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    scroll_id UUID NOT NULL REFERENCES scrolls(id) ON DELETE CASCADE,
    sender_id UUID NOT NULL REFERENCES adventurers(id) ON DELETE CASCADE,
    reply_to_id UUID REFERENCES whispers(id) ON DELETE SET NULL,
    content TEXT NOT NULL,
    message_type VARCHAR(20) NOT NULL DEFAULT 'text',
    is_read BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    edited_at TIMESTAMP,
    CHECK (message_type IN ('text', 'image', 'file'))
);

CREATE OR REPLACE FUNCTION check_whisper_sender_in_scroll()
RETURNS TRIGGER AS $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM scrolls s
        WHERE s.id = NEW.scroll_id
          AND (s.initiator_id = NEW.sender_id OR s.recipient_id = NEW.sender_id)
    ) THEN
        RAISE EXCEPTION 'sender must be a participant of the scroll';
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_whispers_sender_in_scroll
    BEFORE INSERT OR UPDATE OF scroll_id, sender_id ON whispers
    FOR EACH ROW
    EXECUTE FUNCTION check_whisper_sender_in_scroll();
