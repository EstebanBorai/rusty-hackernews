CREATE TRIGGER trigger_delete_old_previews
    AFTER INSERT ON previews
    EXECUTE PROCEDURE delete_old_previews();
