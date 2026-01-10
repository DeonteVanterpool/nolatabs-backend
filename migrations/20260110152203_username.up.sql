-- Add up migration script here
ALTER TABLE users ADD COLUMN IF NOT EXISTS username TEXT;
