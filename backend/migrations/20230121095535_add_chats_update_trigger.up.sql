CREATE TRIGGER update_chats_updated_at
    BEFORE UPDATE
    ON
        chats
    FOR EACH ROW
    EXECUTE PROCEDURE update_updated_at();
