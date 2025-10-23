use crate::database::{models::*, migrations::MigrationRunner};
use crate::{Error, Result};
use chrono::Utc;
use rusqlite::{params, Connection, Row};
use uuid::Uuid;
use std::path::Path;

/// Database repository for password management
pub struct PasswordRepository {
    conn: Connection,
}

impl PasswordRepository {
    /// Create a new repository with database at given path
    pub fn new<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        let conn = Connection::open(db_path.as_ref())?;
        
        // Enable foreign keys
        conn.execute("PRAGMA foreign_keys = ON", [])?;
        
        let repo = Self { conn };
        
        // Run migrations
        let migration_runner = MigrationRunner::new(&repo.conn);
        migration_runner.migrate()?;
        
        Ok(repo)
    }

    /// Initialize vault with master password hash and salt
    pub fn initialize_vault(&self, salt: Vec<u8>, password_hash: Vec<u8>) -> Result<()> {
        let now = Utc::now().to_rfc3339();
        
        self.conn.execute(
            "INSERT INTO vault_metadata (id, created_at, last_access, schema_version, salt, password_hash)
             VALUES (1, ?1, ?2, 1, ?3, ?4)",
            params![now, now, salt, password_hash],
        )?;
        
        Ok(())
    }

    /// Check if vault is initialized
    pub fn is_initialized(&self) -> Result<bool> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM vault_metadata WHERE id = 1",
            [],
            |row| row.get(0),
        )?;
        
        Ok(count > 0)
    }

    /// Get vault metadata
    pub fn get_vault_metadata(&self) -> Result<VaultMetadata> {
        self.conn.query_row(
            "SELECT created_at, last_access, schema_version, salt, password_hash
             FROM vault_metadata WHERE id = 1",
            [],
            |row| {
                Ok(VaultMetadata {
                    created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(0)?)
                        .map_err(|_| rusqlite::Error::InvalidColumnType(0, "created_at".to_string(), rusqlite::types::Type::Text))?
                        .with_timezone(&Utc),
                    last_access: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(1)?)
                        .map_err(|_| rusqlite::Error::InvalidColumnType(1, "last_access".to_string(), rusqlite::types::Type::Text))?
                        .with_timezone(&Utc),
                    schema_version: row.get(2)?,
                    salt: row.get(3)?,
                    password_hash: row.get(4)?,
                })
            },
        )
        .map_err(Error::from)
    }

    /// Update last access time
    pub fn update_last_access(&self) -> Result<()> {
        let now = Utc::now().to_rfc3339();
        
        self.conn.execute(
            "UPDATE vault_metadata SET last_access = ?1 WHERE id = 1",
            params![now],
        )?;
        
        Ok(())
    }

    /// Add a new password entry
    pub fn add_entry(&self, entry: &PasswordEntry, encrypted_password: &[u8]) -> Result<()> {
        self.conn.execute(
            "INSERT INTO password_entries 
             (id, title, username, encrypted_password, url, notes, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                entry.id.to_string(),
                entry.title,
                entry.username,
                encrypted_password,
                entry.url,
                entry.notes,
                entry.created_at.to_rfc3339(),
                entry.updated_at.to_rfc3339(),
            ],
        )?;
        
        Ok(())
    }

    /// Get a password entry by ID
    pub fn get_entry_by_id(&self, id: &Uuid) -> Result<(PasswordEntry, Vec<u8>)> {
        self.conn.query_row(
            "SELECT id, title, username, encrypted_password, url, notes, created_at, updated_at
             FROM password_entries WHERE id = ?1",
            params![id.to_string()],
            Self::row_to_entry_with_encrypted_password,
        )
        .map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => Error::EntryNotFound(id.to_string()),
            _ => Error::from(e),
        })
    }

    /// Get a password entry by title
    pub fn get_entry_by_title(&self, title: &str) -> Result<(PasswordEntry, Vec<u8>)> {
        self.conn.query_row(
            "SELECT id, title, username, encrypted_password, url, notes, created_at, updated_at
             FROM password_entries WHERE title = ?1",
            params![title],
            Self::row_to_entry_with_encrypted_password,
        )
        .map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => Error::EntryNotFound(title.to_string()),
            _ => Error::from(e),
        })
    }

    /// List all password entries (without encrypted passwords)
    pub fn list_entries(&self) -> Result<Vec<PasswordEntry>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, username, encrypted_password, url, notes, created_at, updated_at
             FROM password_entries ORDER BY title"
        )?;
        
        let entries = stmt.query_map([], |row| {
            Self::row_to_entry(row)
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;
        
        Ok(entries)
    }

    /// Search entries by query
    pub fn search_entries(&self, query: &str) -> Result<Vec<PasswordEntry>> {
        let mut stmt = self.conn.prepare(
            "SELECT e.id, e.title, e.username, e.encrypted_password, e.url, e.notes, e.created_at, e.updated_at
             FROM password_entries e
             WHERE e.title LIKE ?1 OR e.username LIKE ?1 OR e.url LIKE ?1 OR e.notes LIKE ?1
             ORDER BY e.title"
        )?;
        
        let search_pattern = format!("%{}%", query);
        let entries = stmt.query_map([&search_pattern], |row| {
            Self::row_to_entry(row)
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;
        
        Ok(entries)
    }

    /// Update a password entry
    pub fn update_entry(&self, entry: &PasswordEntry, encrypted_password: &[u8]) -> Result<()> {
        let updated = self.conn.execute(
            "UPDATE password_entries 
             SET title = ?1, username = ?2, encrypted_password = ?3, url = ?4, notes = ?5, updated_at = ?6
             WHERE id = ?7",
            params![
                entry.title,
                entry.username,
                encrypted_password,
                entry.url,
                entry.notes,
                entry.updated_at.to_rfc3339(),
                entry.id.to_string(),
            ],
        )?;
        
        if updated == 0 {
            return Err(Error::EntryNotFound(entry.id.to_string()));
        }
        
        Ok(())
    }

    /// Delete a password entry
    pub fn delete_entry(&self, id: &Uuid) -> Result<()> {
        let deleted = self.conn.execute(
            "DELETE FROM password_entries WHERE id = ?1",
            params![id.to_string()],
        )?;
        
        if deleted == 0 {
            return Err(Error::EntryNotFound(id.to_string()));
        }
        
        Ok(())
    }

    /// Delete entry by title
    pub fn delete_entry_by_title(&self, title: &str) -> Result<()> {
        let deleted = self.conn.execute(
            "DELETE FROM password_entries WHERE title = ?1",
            params![title],
        )?;
        
        if deleted == 0 {
            return Err(Error::EntryNotFound(title.to_string()));
        }
        
        Ok(())
    }

    /// Helper function to convert row to PasswordEntry
    fn row_to_entry(row: &Row) -> rusqlite::Result<PasswordEntry> {
        let id_str: String = row.get(0)?;
        let id = Uuid::parse_str(&id_str)
            .map_err(|_| rusqlite::Error::InvalidColumnType(0, "id".to_string(), rusqlite::types::Type::Text))?;
            
        let created_at_str: String = row.get(6)?;
        let created_at = chrono::DateTime::parse_from_rfc3339(&created_at_str)
            .map_err(|_| rusqlite::Error::InvalidColumnType(6, "created_at".to_string(), rusqlite::types::Type::Text))?
            .with_timezone(&Utc);
            
        let updated_at_str: String = row.get(7)?;
        let updated_at = chrono::DateTime::parse_from_rfc3339(&updated_at_str)
            .map_err(|_| rusqlite::Error::InvalidColumnType(7, "updated_at".to_string(), rusqlite::types::Type::Text))?
            .with_timezone(&Utc);
        
        Ok(PasswordEntry {
            id,
            title: row.get(1)?,
            username: row.get(2)?,
            password: SecureString::new(String::new()), // Empty for list operations
            url: row.get(4)?,
            notes: row.get(5)?,
            created_at,
            updated_at,
        })
    }

    /// Helper function to convert row to PasswordEntry with encrypted password
    fn row_to_entry_with_encrypted_password(row: &Row) -> rusqlite::Result<(PasswordEntry, Vec<u8>)> {
        let entry = Self::row_to_entry(row)?;
        let encrypted_password: Vec<u8> = row.get(3)?;
        Ok((entry, encrypted_password))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_repository_creation() {
        let temp_file = NamedTempFile::new().unwrap();
        let repo = PasswordRepository::new(temp_file.path()).unwrap();
        
        assert!(!repo.is_initialized().unwrap());
    }

    #[test]
    fn test_vault_initialization() {
        let temp_file = NamedTempFile::new().unwrap();
        let repo = PasswordRepository::new(temp_file.path()).unwrap();
        
        let salt = vec![1, 2, 3, 4];
        let password_hash = vec![5, 6, 7, 8];
        
        repo.initialize_vault(salt.clone(), password_hash.clone()).unwrap();
        assert!(repo.is_initialized().unwrap());
        
        let metadata = repo.get_vault_metadata().unwrap();
        assert_eq!(metadata.salt, salt);
        assert_eq!(metadata.password_hash, password_hash);
    }
}
