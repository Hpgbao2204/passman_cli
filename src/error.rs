use thiserror::Error;

/// Application error types
#[derive(Error, Debug)]
pub enum Error {
    /// Database related errors
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    /// Cryptographic errors
    #[error("Crypto error: {0}")]
    Crypto(String),

    /// Configuration errors
    #[error("Configuration error: {0}")]
    Config(#[from] config::ConfigError),

    /// IO errors
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Serialization errors
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Invalid input errors
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// Authentication errors
    #[error("Authentication failed: {0}")]
    Authentication(String),

    /// Entry not found
    #[error("Entry not found: {0}")]
    EntryNotFound(String),

    /// Clipboard errors
    #[error("Clipboard error: {0}")]
    Clipboard(String),

    /// Password generation errors
    #[error("Password generation error: {0}")]
    PasswordGeneration(String),

    /// Vault not initialized
    #[error("Vault not initialized. Run 'passman init' first")]
    VaultNotInitialized,

    /// Vault already exists
    #[error("Vault already exists")]
    VaultAlreadyExists,
}

/// Application result type
pub type Result<T> = std::result::Result<T, Error>;

impl From<ring::error::Unspecified> for Error {
    fn from(err: ring::error::Unspecified) -> Self {
        Error::Crypto(format!("Ring crypto error: {}", err))
    }
}

impl From<argon2::Error> for Error {
    fn from(err: argon2::Error) -> Self {
        Error::Crypto(format!("Argon2 error: {}", err))
    }
}

impl From<argon2::password_hash::Error> for Error {
    fn from(err: argon2::password_hash::Error) -> Self {
        Error::Crypto(format!("Password hash error: {}", err))
    }
}
