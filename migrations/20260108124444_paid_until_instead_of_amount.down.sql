-- Add down migration script here
BEGIN;

ALTER TABLE credit DROP COLUMN paid_until;
ALTER TABLE credit ADD COLUMN amount INT NOT NULL CHECK (amount >= 0);

COMMIT;
