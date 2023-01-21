CREATE TRIGGER update_projects_updated_at
    BEFORE UPDATE
    ON
        projects
    FOR EACH ROW
    EXECUTE PROCEDURE update_updated_at();
