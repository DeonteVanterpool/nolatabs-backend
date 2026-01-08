-- Add down migration script here
ALTER TABLE commit DROP COLUMN IF EXISTS parents;
