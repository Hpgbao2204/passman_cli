-- Initial database schema for PassMan-CLI
-- Version 1: Basic password storage with encryption

-- Vault metadata table
CREATE TABLE vault_metadata (
    id INTEGER PRIMARY KEY CHECK (id = 1), -- Ensure only one row
    created_at TEXT NOT NULL,
    last_access TEXT NOT NULL,
    schema_version INTEGER NOT NULL DEFAULT 1,
    salt BLOB NOT NULL,
    password_hash BLOB NOT NULL
);

-- Password entries table
CREATE TABLE password_entries (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    username TEXT NOT NULL,
    encrypted_password BLOB NOT NULL,
    url TEXT,
    notes TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Create indexes for better performance
CREATE INDEX idx_password_entries_title ON password_entries(title);
CREATE INDEX idx_password_entries_username ON password_entries(username);
CREATE INDEX idx_password_entries_url ON password_entries(url);
CREATE INDEX idx_password_entries_created_at ON password_entries(created_at);
CREATE INDEX idx_password_entries_updated_at ON password_entries(updated_at);

-- Full-text search virtual table for entries
CREATE VIRTUAL TABLE password_entries_fts USING fts5(
    id UNINDEXED,
    title,
    username,
    url,
    notes,
    content='password_entries',
    content_rowid='rowid'
);

-- Triggers to keep FTS table in sync
CREATE TRIGGER password_entries_fts_insert AFTER INSERT ON password_entries BEGIN
    INSERT INTO password_entries_fts(id, title, username, url, notes)
    VALUES (new.id, new.title, new.username, new.url, new.notes);
END;

CREATE TRIGGER password_entries_fts_delete AFTER DELETE ON password_entries BEGIN
    DELETE FROM password_entries_fts WHERE id = old.id;
END;

CREATE TRIGGER password_entries_fts_update AFTER UPDATE ON password_entries BEGIN
    DELETE FROM password_entries_fts WHERE id = old.id;
    INSERT INTO password_entries_fts(id, title, username, url, notes)
    VALUES (new.id, new.title, new.username, new.url, new.notes);
END;
