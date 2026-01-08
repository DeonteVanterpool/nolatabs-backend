-- Add down migration script here
ALTER TABLE commits DROP COLUMN IF EXISTS parents;
