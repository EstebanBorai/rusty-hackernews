-- Add migration script here
ALTER TABLE users
ADD COLUMN token TEXT;
