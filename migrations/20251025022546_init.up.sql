BEGIN;

-- Table containing basic user information
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY NOT NULL,
    email TEXT UNIQUE NOT NULL CHECK (email ~ '^.+@.+$'),
    created TIMESTAMP NOT NULL DEFAULT NOW(),
    deleted TIMESTAMP
);

CREATE TABLE IF NOT EXISTS subscription_plans (
    id TEXT PRIMARY KEY NOT NULL,
    description TEXT,
    price_cents_per_month INT NOT NULL CHECK (price_cents_per_month >= 0)
);

INSERT INTO subscription_plans (id, description, price_cents_per_month) VALUES 
('cloud sync', 'Access to cloud sync features', 200), 
('sync collaborate', 'Access to premium support features', 300);

CREATE TABLE IF NOT EXISTS credit (
    amount INT NOT NULL CHECK (amount >= 0),
    user_id UUID PRIMARY KEY REFERENCES users(id) NOT NULL,
    plan_id TEXT REFERENCES subscription_plans(id) NOT NULL
);

CREATE TABLE IF NOT EXISTS payment_log (
    payment_id TEXT PRIMARY KEY NOT NULL, -- e.g., Stripe payment intent ID
    user_id UUID NOT NULL REFERENCES users(id),
    amount_cents INT NOT NULL CHECK (amount_cents >= 0),
    payment_date TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS command_styles (
    id TEXT PRIMARY KEY NOT NULL,
    description TEXT
);

INSERT INTO command_styles (id, description) VALUES 
('terminal style', 'Git / terminal style commands for power users'), 
('plain-english style', 'Plain English style commands for ease of use');

CREATE TABLE IF NOT EXISTS autopush_options (
    id TEXT PRIMARY KEY NOT NULL,
    description TEXT
);

CREATE TABLE IF NOT EXISTS autopull_options (
    id TEXT PRIMARY KEY NOT NULL,
    description TEXT
);

INSERT INTO autopull_options (id, description) VALUES 
('timer', 'Automatically pull changes at regular time intervals'), 
('on', 'Automatically pull changes when they are available'), 
('off', 'Do not automatically pull changes');

CREATE TABLE IF NOT EXISTS autocommit_options (
    id TEXT PRIMARY KEY NOT NULL,
    description TEXT
);

INSERT INTO autocommit_options (id, description) VALUES 
('timer', 'Automatically commit changes at regular time intervals'), 
('count', 'Automatically commit changes after a certain number of changes'), 
('off', 'Do not automatically commit changes');

INSERT INTO autopush_options (id, description) VALUES 
('timer', 'Automatically push changes at regular time intervals'), 
('count', 'Automatically push changes after a certain number of commits'), 
('off', 'Do not automatically push changes');

CREATE TABLE IF NOT EXISTS user_settings (
    user_id UUID PRIMARY KEY NOT NULL REFERENCES users(id),
    command_style TEXT NOT NULL REFERENCES command_styles(id),
    autopush_option TEXT NOT NULL REFERENCES autopush_options(id),
    autopush_duration INTERVAL,
    autopush_interval_count INT,
    autopull_option TEXT NOT NULL REFERENCES autopull_options(id),
    autopull_duration INTERVAL,
    autocommit_option TEXT NOT NULL REFERENCES autocommit_options(id),
    autocommit_duration INTERVAL,
    autocommit_interval_count INT
);

CREATE TABLE IF NOT EXISTS message_types (
    id TEXT PRIMARY KEY NOT NULL,
    description TEXT
);

INSERT INTO message_types (id, description) VALUES 
('proposal', 'MLS Proposal Message'),
('commit', 'MLS Commit Message'),
('welcome', 'MLS Welcome Message'),
('application', 'MLS Application Message');

CREATE TABLE IF NOT EXISTS repo_permissions (
    id TEXT PRIMARY KEY NOT NULL,
    description TEXT
);

INSERT INTO repo_permissions (id, description) VALUES 
('viewer', 'Able to view the tabs in the current repo'), 
('contributor', 'Able to suggest changes to the current repo'), 
('editor', 'Able to approve and make changes to the current repo'),
('admin', 'Able to manage repo settings and permissions');

CREATE TABLE IF NOT EXISTS repos (
    id TEXT UNIQUE NOT NULL,
    owner UUID REFERENCES users(id) NOT NULL,
    -- owner of a repo must be a paying user, not an anonymous client 
    name TEXT PRIMARY KEY NOT NULL CHECK (name ~* '^((?!\/).)*$'), -- regular expression does not contain '/',
    deleted TIMESTAMP,
    UNIQUE (owner, name)
);

CREATE TABLE IF NOT EXISTS mls_clients (
    id UUID PRIMARY KEY NOT NULL,
    user_id UUID REFERENCES users,
    deleted TIMESTAMP
);

CREATE TABLE IF NOT EXISTS key_packages (
    client_id UUID NOT NULL REFERENCES mls_clients(id),
    key_package BYTEA NOT NULL,
    expiration_date TIMESTAMP,
    deleted TIMESTAMP
);

CREATE TABLE IF NOT EXISTS client_repos (
    client_id UUID NOT NULL REFERENCES mls_clients(id),
    repo_id TEXT NOT NULL REFERENCES repos(id),
    permission_level TEXT NOT NULL REFERENCES repo_permissions(id),
    delegation_level TEXT REFERENCES repo_permissions(id), -- the level that this client can delegate to others (must be less than or equal to permission_level)
    deleted TIMESTAMP,

    CONSTRAINT valid_delegation_level CHECK (
    (permission_level = 'viewer' AND delegation_level IS NULL) OR
    (permission_level = 'contributor' AND delegation_level IN ('viewer', NULL)) OR
    (permission_level = 'editor' AND delegation_level IN ('viewer', 'contributor', NULL)) OR
    (permission_level = 'admin' AND delegation_level IN ('viewer', 'contributor', 'editor', NULL)))
);

-- MLS Encoded unicast Messages table
CREATE TABLE IF NOT EXISTS unicast_messages (
    id UUID PRIMARY KEY NOT NULL,
    mls_data BYTEA NOT NULL,
    message_type TEXT NOT NULL REFERENCES message_types(id),
    created TIMESTAMP NOT NULL DEFAULT NOW(),
    sender_id UUID NOT NULL REFERENCES mls_clients(id),
    recipient_id UUID NOT NULL REFERENCES mls_clients(id),
    deleted TIMESTAMP
);

-- MLS Encoded broadcast Messages table
CREATE TABLE IF NOT EXISTS broadcast_messages (
    id UUID PRIMARY KEY NOT NULL,
    mls_data BYTEA NOT NULL,
    message_type TEXT NOT NULL REFERENCES message_types(id),
    created TIMESTAMP NOT NULL DEFAULT NOW(),
    sender_id UUID NOT NULL REFERENCES mls_clients(id),
    repo_id TEXT NOT NULL REFERENCES repos(id),
    deleted TIMESTAMP
);

CREATE TABLE IF NOT EXISTS broadcast_messages_read_receipts (
    message_id UUID NOT NULL REFERENCES broadcast_messages(id) ON DELETE CASCADE,
    reader_id UUID NOT NULL REFERENCES mls_clients(id),
    read_at TIMESTAMP NOT NULL,
    UNIQUE (message_id, reader_id)
);

CREATE TABLE IF NOT EXISTS unicast_messages_read_receipts (
    message_id UUID NOT NULL REFERENCES unicast_messages(id) ON DELETE CASCADE,
    reader_id UUID NOT NULL REFERENCES mls_clients(id),
    read_at TIMESTAMP NOT NULL,
    UNIQUE (message_id, reader_id)
);

CREATE TABLE IF NOT EXISTS commits (
    id TEXT PRIMARY KEY NOT NULL, -- commit hash
    changes UUID REFERENCES broadcast_messages(id) ON DELETE SET NULL,
    message UUID REFERENCES broadcast_messages(id) ON DELETE SET NULL,
    repo_id TEXT NOT NULL REFERENCES repos(id),
    created TIMESTAMP NOT NULL
);

CREATE TABLE IF NOT EXISTS blob_server_backups (
    blob_server_id UUID NOT NULL,
    related_commit TEXT NOT NULL REFERENCES commits(id),
    deleted TIMESTAMP
);

COMMIT;
