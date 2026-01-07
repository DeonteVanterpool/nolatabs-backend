-- Table containing basic user information
CREATE TABLE users (
    id SERIAL PRIMARY KEY NOT NULL,
    email TEXT UNIQUE NOT NULL CHECK (email ~ '^.+@.+$'),
    created TIMESTAMP NOT NULL DEFAULT NOW(),
    deleted TIMESTAMP,
);

CREATE TABLE subscriptions (
    expires TIMESTAMP NOT NULL,
    user_id INT REFERENCES users(id) NOT NULL,
);

CREATE TABLE command_styles (
    id TEXT PRIMARY KEY NOT NULL,
    description TEXT,
);

INSERT INTO command_styles (id, description) VALUES 
('terminal style', 'Git / terminal style commands for power users'), 
('plain-english style', 'Plain English style commands for ease of use');

CREATE TABLE autopush_options (
    id TEXT PRIMARY KEY NOT NULL,
    description TEXT,
);

CREATE TABLE autopull_options (
    id TEXT PRIMARY KEY NOT NULL,
    description TEXT,
);

INSERT INTO autopull_options (id, description) VALUES 
('timer', 'Automatically pull changes at regular time intervals'), 
('on', 'Automatically pull changes when they are available'), 
('off', 'Do not automatically pull changes');

CREATE TABLE autocommit_options (
    id TEXT PRIMARY KEY NOT NULL,
    description TEXT,
);

INSERT INTO autocommit_options (id, description) VALUES 
('timer', 'Automatically commit changes at regular time intervals'), 
('count', 'Automatically commit changes after a certain number of changes'), 
('off', 'Do not automatically commit changes');

INSERT INTO autopush_options (id, description) VALUES 
('timer', 'Automatically push changes at regular time intervals'), 
('count', 'Automatically push changes after a certain number of commits'), 
('off', 'Do not automatically push changes');

CREATE TABLE user_settings (
    user_id INT PRIMARY KEY NOT NULL REFERENCES users(id),
    command_style TEXT NOT NULL REFERENCES command_styles(id),
    autopush_option TEXT NOT NULL REFERENCES autopush_options(id),
    autopush_duration INTERVAL,
    autopush_interval_count INT,
    autopull_option TEXT NOT NULL REFERENCES autopull_options(id),
    autopull_duration INTERVAL,
    autocommit_option TEXT NOT NULL REFERENCES autocommit_options(id),
    autocommit_duration INTERVAL,
    autocommit_interval_count INT,
);

CREATE TABLE message_types (
    id TEXT PRIMARY KEY NOT NULL,
    description TEXT,
);

INSERT INTO message_types (id, description) VALUES 
('proposal', 'MLS Proposal Message'),
('commit', 'MLS Commit Message'),
('welcome', 'MLS Welcome Message'),
('application', 'MLS Application Message');

CREATE TABLE repo_permissions (
    id TEXT PRIMARY KEY NOT NULL,
    description TEXT,
);

INSERT INTO repo_permissions (description) VALUES 
('viewer', 'Able to view the tabs in the current repo'), 
('contributor', 'Able to suggest changes to the current repo'), 
('editor', 'Able to approve and make changes to the current repo'),
('admin', 'Able to manage repo settings and permissions');

CREATE TABLE repos (
    id TEXT UNIQUE NOT NULL,
    owner INT REFERENCES users(id) NOT NULL,
    -- owner of a repo must be a paying user, not an anonymous client 
    name TEXT PRIMARY KEY NOT NULL CHECK (name ~* '^((?!\/).)*$') -- regular expression does not contain '/',
    unique(owner, name),
    deleted TIMESTAMP,
);

CREATE TABLE client_repos (
    client_id INT NOT NULL REFERENCES mls_clients(id),
    repo_id TEXT NOT NULL REFERENCES repos(id),
    permission_level INT NOT NULL REFERENCES repo_permissions(id),
    delegation_level INT REFERENCES repo_permissions(id), -- the level that this client can delegate to others (must be less than or equal to permission_level)
    deleted TIMESTAMP,
);

CREATE TABLE key_packages (
    client_id INT NOT NULL REFERENCES mls_clients(id),
    key_package JSONB NOT NULL,
    expiration_date TIMESTAMP,
    deleted TIMESTAMP,
);

CREATE TABLE mls_clients (
    id SERIAL PRIMARY KEY NOT NULL,
    user_id INT REFERENCES users,
    deleted TIMESTAMP,
);

-- MLS Encoded unicast Messages table
CREATE TABLE unicast_messages (
    id SERIAL PRIMARY KEY NOT NULL,
    mls_data BYTEA NOT NULL,
    message_type TEXT NOT NULL REFERENCES message_types(id),
    created TIMESTAMP NOT NULL DEFAULT NOW(),
    sender_id INT NOT NULL REFERENCES mls_clients(id),
    recipient_id INT NOT NULL REFERENCES mls_clients(id),
    deleted TIMESTAMP,
);

-- MLS Encoded broadcast Messages table
CREATE TABLE broadcast_messages (
    id SERIAL PRIMARY KEY NOT NULL,
    mls_data BYTEA NOT NULL,
    message_type TEXT NOT NULL REFERENCES message_types(id),
    created TIMESTAMP NOT NULL DEFAULT NOW(),
    sender_id INT NOT NULL REFERENCES mls_clients(id),
    repo_id TEXT NOT NULL REFERENCES repos(id),
    deleted TIMESTAMP,
);

CREATE TABLE broadcast_messages_read_receipts (
    message_id INT NOT NULL REFERENCES broadcast_messages(id) ON DELETE CASCADE,
    reader_id INT NOT NULL REFERENCES mls_clients(id),
    read_at TIMESTAMP NOT NULL,
);

CREATE TABLE unicast_messages_read_receipts (
    message_id INT NOT NULL REFERENCES unicast_messages(id) ON DELETE CASCADE,
    reader_id INT NOT NULL REFERENCES mls_clients(id),
    read_at TIMESTAMP NOT NULL,
);

CREATE TABLE commits (
    id TEXT PRIMARY KEY NOT NULL, -- commit hash
    changes INT REFERENCES broadcast_messages(id) ON DELETE SET NULL,
    message INT REFERENCES broadcast_messages(id) ON DELETE SET NULL,
    repo_id TEXT NOT NULL REFERENCES repos(id),
    created TIMESTAMP NOT NULL,
);

CREATE TABLE blob_server_backups (
    blob_server_id INT NOT NULL,
    related_commit TEXT NOT NULL REFERENCES commits(id),
    deleted TIMESTAMP,
);
