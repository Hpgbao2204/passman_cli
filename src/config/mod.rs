use crate::{Error, Result, APP_NAME, CONFIG_FILE_NAME, DEFAULT_DB_NAME};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Database file path
    pub database_path: PathBuf,
    /// Clipboard timeout in seconds (0 = no timeout)
    pub clipboard_timeout: u64,
    /// Password generation settings
    pub password_generation: PasswordGenerationConfig,
    /// Security settings
    pub security: SecurityConfig,
}

/// Password generation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordGenerationConfig {
    /// Default password length
    pub default_length: u32,
    /// Include uppercase letters
    pub include_uppercase: bool,
    /// Include lowercase letters
    pub include_lowercase: bool,
    /// Include numbers
    pub include_numbers: bool,
    /// Include symbols
    pub include_symbols: bool,
    /// Custom symbol set
    pub symbol_set: String,
}

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Session timeout in minutes (0 = no timeout)
    pub session_timeout: u64,
    /// Maximum login attempts before lockout
    pub max_login_attempts: u32,
    /// Lockout duration in minutes
    pub lockout_duration: u64,
}

impl Default for Config {
    fn default() -> Self {
        let mut database_path = dirs::config_dir()
            .or_else(|| dirs::home_dir())
            .unwrap_or_else(|| PathBuf::from("."));

        database_path.push(APP_NAME);
        database_path.push(DEFAULT_DB_NAME);

        Self {
            database_path,
            clipboard_timeout: 30, // 30 seconds
            password_generation: PasswordGenerationConfig::default(),
            security: SecurityConfig::default(),
        }
    }
}

impl Default for PasswordGenerationConfig {
    fn default() -> Self {
        Self {
            default_length: 16,
            include_uppercase: true,
            include_lowercase: true,
            include_numbers: true,
            include_symbols: true,
            symbol_set: "!@#$%^&*()-_=+[]{}|;:,.<>?".to_string(),
        }
    }
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            session_timeout: 15, // 15 minutes
            max_login_attempts: 3,
            lockout_duration: 5, // 5 minutes
        }
    }
}

impl Config {
    /// Load configuration from file or create default
    pub fn load() -> Result<Self> {
        let config_path = Self::config_file_path()?;

        if config_path.exists() {
            let contents = std::fs::read_to_string(&config_path)?;
            let config: Config = toml::from_str(&contents)
                .map_err(|e| Error::Config(config::ConfigError::Message(e.to_string())))?;
            Ok(config)
        } else {
            let config = Config::default();
            config.save()?;
            Ok(config)
        }
    }

    /// Save configuration to file
    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_file_path()?;

        // Create config directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let toml_string = toml::to_string_pretty(self)
            .map_err(|e| Error::Config(config::ConfigError::Message(e.to_string())))?;

        std::fs::write(config_path, toml_string)?;
        Ok(())
    }

    /// Get the path to the configuration file
    fn config_file_path() -> Result<PathBuf> {
        let mut config_path = dirs::config_dir()
            .or_else(|| dirs::home_dir())
            .ok_or_else(|| {
                Error::Config(config::ConfigError::Message(
                    "Cannot determine config directory".to_string(),
                ))
            })?;

        config_path.push(APP_NAME);
        config_path.push(CONFIG_FILE_NAME);
        Ok(config_path)
    }

    /// Get the directory containing the database
    pub fn database_dir(&self) -> Option<&std::path::Path> {
        self.database_path.parent()
    }

    /// Ensure the database directory exists
    pub fn ensure_database_dir(&self) -> Result<()> {
        if let Some(db_dir) = self.database_dir() {
            std::fs::create_dir_all(db_dir)?;
        }
        Ok(())
    }
}
