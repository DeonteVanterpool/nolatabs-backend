-- Add down migration script here
ALTER TABLE unicast_messages_read_receipts ADD COLUMN IF EXISTS reader_id;
