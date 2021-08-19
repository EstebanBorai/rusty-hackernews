-- Add migration script here
ALTER TABLE previews
RENAME COLUMN url TO url_hash;
