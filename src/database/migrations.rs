use crate::Result;
use rusqlite::Connection;

/// Database migration management
pub struct Migration {
    pub version: u32,
    pub description: &'static str,
    pub sql: &'static str,
}

/// All available migrations
pub const MIGRATIONS: &[Migration] = &[
    Migration {
        version: 1,
        description: "Initial schema",
        sql: r#"
-- Initial database schema for PassMan-CLI
-- Version 1: Basic password storage with encryption

-- Vault metadata table
CREATE TABLE vault_metadata (
    id INTEGER PRIMARY KEY CHECK (id = 1),
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
"#,
    },
];

/// Migration runner
pub struct MigrationRunner<'a> {
    conn: &'a Connection,
}

impl<'a> MigrationRunner<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    /// Run all pending migrations
    pub fn migrate(&self) -> Result<()> {
        // Create migration table if it doesn't exist
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS migrations (
                version INTEGER PRIMARY KEY,
                description TEXT NOT NULL,
                applied_at TEXT NOT NULL
            )",
            [],
        )?;

        // Get current version
        let current_version = self.get_current_version()?;
        
        // Apply pending migrations
        for migration in MIGRATIONS {
            if migration.version > current_version {
                println!("Applying migration {}: {}", migration.version, migration.description);
                self.apply_migration(migration)?;
            }
        }

        Ok(())
    }

    fn get_current_version(&self) -> Result<u32> {
        let version = self.conn
            .query_row(
                "SELECT MAX(version) FROM migrations",
                [],
                |row| row.get::<_, Option<u32>>(0),
            )
            .unwrap_or(Some(0))
            .unwrap_or(0);
        
        Ok(version)
    }

    fn apply_migration(&self, migration: &Migration) -> Result<()> {
        // Start transaction
        let tx = self.conn.unchecked_transaction()?;
        
        // Execute migration SQL
        tx.execute_batch(migration.sql)?;
        
        // Record migration
        tx.execute(
            "INSERT INTO migrations (version, description, applied_at) VALUES (?1, ?2, datetime('now'))",
            rusqlite::params![migration.version, migration.description],
        )?;
        
        // Commit transaction
        tx.commit()?;
        
        Ok(())
    }
}
