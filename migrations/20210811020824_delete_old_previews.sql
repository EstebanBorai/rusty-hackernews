CREATE FUNCTION delete_old_previews() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
  DELETE FROM previews WHERE created_at < NOW() - INTERVAL '2 days';
  RETURN NULL;
END;
$$;
