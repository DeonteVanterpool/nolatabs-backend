-- Table containing basic user information
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email TEXT,
    created TIMESTAMP,
);

CREATE TABLE subscriptions (
    expires TIMESTAMP,
    user_id INT REFERENCES users(id),
);

CREATE TABLE message_types (
    id TEXT PRIMARY KEY,
    description TEXT,
);

CREATE TABLE repos (
    id SERIAL PRIMARY KEY,
    owner INT REFERENCES users(id),
    name TEXT PRIMARY KEY,
);

CREATE TABLE commits (
    id TEXT PRIMARY KEY,
    changes INT REFERENCES messages(id),
    repo INT REFERENCES repos(id),
    created TIMESTAMP,
);

CREATE TABLE repo_permissions (
    id SERIAL PRIMARY KEY,
    description TEXT,
);

INSERT INTO repo_permissions (description) VALUES ('read-only'), ('read-write');

CREATE TABLE user_repos (
    user_id INT REFERENCES users(id),
    repos INT REFERENCES repos(id),
    permission_level INT REFERENCES repo_permissions(id),
);

CREATE TABLE blob_server_backups (
    blob_server_id number,
    related_commit text,
);

-- MLS Encoded Messages table
CREATE TABLE messages (
    id SERIAL PRIMARY KEY,
    mls_data BYTEA NOT NULL,
    message_type TEXT NOT NULL REFERENCES message_types(id),
    created TIMESTAMP NOT NULL DEFAULT NOW(),
    sender_id INT NOT NULL REFERENCES users(id)
);
