-- Add up migration script here
ALTER TABLE commit ADD COLUMN IF NOT EXISTS parents TEXT[];
