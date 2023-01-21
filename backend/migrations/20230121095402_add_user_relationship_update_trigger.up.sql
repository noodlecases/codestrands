CREATE TRIGGER update_user_relationship_updated_at
    BEFORE UPDATE
    ON
        user_relationships
    FOR EACH ROW
    EXECUTE PROCEDURE update_updated_at();
