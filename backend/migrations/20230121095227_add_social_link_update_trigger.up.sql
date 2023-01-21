CREATE TRIGGER update_social_link_updated_at
    BEFORE UPDATE
    ON
        social_links
    FOR EACH ROW
    EXECUTE PROCEDURE update_updated_at();
