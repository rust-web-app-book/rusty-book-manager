-- Add down migration script here
DROP TRIGGER IF EXISTS books_updated_at_trigger ON books;
DROP TABLE IF EXISTS books;

DROP FUNCTION set_updated_at;
