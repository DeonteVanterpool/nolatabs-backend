-- Table containing basic user information
CREATE TABLE users (
    id SERIAL PRIMARY KEY NOT NULL,
    email TEXT UNIQUE NOT NULL,
    created TIMESTAMP,

    CONSTRAINT proper_email CHECK (email ~ '^.+@.+$'),
);

CREATE TABLE subscriptions (
    expires TIMESTAMP NOT NULL,
    user_id INT REFERENCES users(id),
);

-- types: proposal, commit, welcome, application
CREATE TABLE message_types (
    id TEXT PRIMARY KEY,
    description TEXT,
);

INSERT INTO message_types (id, description) VALUES 
('proposal', 'MLS Proposal Message'),
('commit', 'MLS Commit Message'),
('welcome', 'MLS Welcome Message'),
('application', 'MLS Application Message');

CREATE TABLE repo_permissions (
    id SERIAL PRIMARY KEY NOT NULl,
    description TEXT,
);

INSERT INTO repo_permissions (description) VALUES ('read-only'), ('read-write');

CREATE TABLE repos (
    id TEXT UNIQUE NOT NULL,
    owner INT REFERENCES users(id) NOT NULL CHECK (owner ~* '^((?!\/).)*$'), -- regular expression does not contain '/'
    name TEXT PRIMARY KEY NOT NULL CHECK (owner ~* '^((?!\/).)*$') -- regular expression does not contain '/',
    deleted TIMESTAMP,
);

CREATE TABLE user_repos (
    user_id INT REFERENCES users(id),
    repos INT REFERENCES repos(id),
    permission_level INT REFERENCES repo_permissions(id),
);

CREATE TABLE key_packages (
    user_id INT NOT NULL REFERENCES users,
    key_package JSONB NOT NULL,
    expiration_date TIMESTAMP,
    deleted TIMESTAMP,
);

CREAT TABLE mls_clients (
    id SERIAL PRIMARY KEY NOT NULL,
    user_id INT REFERENCES users,
);

-- MLS Encoded unicast Messages table
CREATE TABLE unicast_messages (
    id SERIAL PRIMARY KEY,
    mls_data BYTEA NOT NULL,
    message_type TEXT NOT NULL REFERENCES message_types(id),
    created TIMESTAMP NOT NULL DEFAULT NOW(),
    sender_id INT NOT NULL REFERENCES mls_clients(id),
    recipient_id INT REFERENCES mls_clients(id),
    recipient_email TEXT
);

-- MLS Encoded broadcast Messages table
CREATE TABLE broadcast_messages (
    id SERIAL PRIMARY KEY,
    mls_data BYTEA NOT NULL,
    message_type TEXT NOT NULL REFERENCES message_types(id),
    created TIMESTAMP NOT NULL DEFAULT NOW(),
    sender_id INT NOT NULL REFERENCES users(id),
    repo_id INT REFERENCES repos(id),
    recipient_email TEXT
);

CREATE TABLE commits (
    id TEXT PRIMARY KEY,
    changes INT REFERENCES broadcast_messages(id),
    message INT REFERENCES broadcast_messages(id),
    repo TEXT REFERENCES repos(id),
    created TIMESTAMP,
);

CREATE TABLE blob_server_backups (
    blob_server_id number,
    related_commit text,
);
