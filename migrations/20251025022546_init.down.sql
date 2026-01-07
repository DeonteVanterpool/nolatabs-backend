BEGIN;

-- Drop all tables in reverse order of creation to avoid dependency issues
DROP TABLE IF EXISTS blob_server_backups;
DROP TABLE IF EXISTS commits;
DROP TABLE IF EXISTS broadcast_messages_read_receipts;
DROP TABLE IF EXISTS unicast_messages_read_receipts;
DROP TABLE IF EXISTS broadcast_messages;
DROP TABLE IF EXISTS unicast_messages;
DROP TABLE IF EXISTS key_packages;
DROP TABLE IF EXISTS client_repos;
DROP TABLE IF EXISTS mls_clients;
DROP TABLE IF EXISTS repos;
DROP TABLE IF EXISTS repo_permissions;
DROP TABLE IF EXISTS message_types;
DROP TABLE IF EXISTS user_settings;
DROP TABLE IF EXISTS autocommit_options;
DROP TABLE IF EXISTS autopull_options;
DROP TABLE IF EXISTS autopush_options;
DROP TABLE IF EXISTS command_styles;
DROP TABLE IF EXISTS subscriptions;
DROP TABLE IF EXISTS users;

COMMIT;
