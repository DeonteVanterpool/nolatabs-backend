-- Add up migration script here
BEGIN;

ALTER TABLE credit DROP COLUMN amount;
ALTER TABLE credit ADD COLUMN paid_until TIMESTAMP NOT NULL;

COMMIT;
