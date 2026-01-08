-- Add up migration script here
ALTER TABLE commits ADD COLUMN IF NOT EXISTS parents TEXT[];
