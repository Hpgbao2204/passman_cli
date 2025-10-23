use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use zeroize::{Zeroize, ZeroizeOnDrop};

/// Password entry in the database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordEntry {
    /// Unique identifier
    pub id: Uuid,
    /// Entry title/name
    pub title: String,
    /// Username/email
    pub username: String,
    /// Encrypted password
    #[serde(skip)]
    pub password: SecureString,
    /// Website URL (optional)
    pub url: Option<String>,
    /// Additional notes (optional)
    pub notes: Option<String>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

/// Secure string that zeros memory on drop
#[derive(Debug, Clone, Zeroize, ZeroizeOnDrop, Default)]
pub struct SecureString(String);

impl SecureString {
    pub fn new(value: String) -> Self {
        Self(value)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(mut self) -> String {
        std::mem::take(&mut self.0)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl From<String> for SecureString {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&str> for SecureString {
    fn from(value: &str) -> Self {
        Self::new(value.to_string())
    }
}

/// Database schema version for migrations
#[derive(Debug, Clone)]
pub struct SchemaVersion {
    pub version: u32,
    pub applied_at: DateTime<Utc>,
}

/// Vault metadata stored in the database
#[derive(Debug, Clone)]
pub struct VaultMetadata {
    /// Vault creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last access timestamp
    pub last_access: DateTime<Utc>,
    /// Schema version
    pub schema_version: u32,
    /// Salt for key derivation
    pub salt: Vec<u8>,
    /// Password verification hash
    pub password_hash: Vec<u8>,
}

impl PasswordEntry {
    /// Create a new password entry
    pub fn new(
        title: String,
        username: String,
        password: SecureString,
        url: Option<String>,
        notes: Option<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            title,
            username,
            password,
            url,
            notes,
            created_at: now,
            updated_at: now,
        }
    }

    /// Update the entry's timestamp
    pub fn touch(&mut self) {
        self.updated_at = Utc::now();
    }
}

impl VaultMetadata {
    /// Create new vault metadata
    pub fn new(salt: Vec<u8>, password_hash: Vec<u8>) -> Self {
        let now = Utc::now();
        Self {
            created_at: now,
            last_access: now,
            schema_version: 1,
            salt,
            password_hash,
        }
    }

    /// Update last access timestamp
    pub fn update_access(&mut self) {
        self.last_access = Utc::now();
    }
}
