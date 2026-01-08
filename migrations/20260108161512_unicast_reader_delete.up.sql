-- Add up migration script here
ALTER TABLE unicast_messages_read_receipts DROP COLUMN IF EXISTS reader_id;
