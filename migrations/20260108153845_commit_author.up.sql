-- Add up migration script here
ALTER TABLE commits ADD COLUMN IF NOT EXISTS author UUID REFERENCES mls_clients(id);
