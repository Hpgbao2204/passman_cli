//! # PassMan-CLI
//!
//! A secure offline password manager CLI tool built with Rust.
//!
//! ## Features
//!
//! - **Offline-first**: No cloud dependencies, all data stored locally
//! - **Secure encryption**: Uses AES-256-GCM with Argon2 key derivation
//! - **SQLCipher**: Encrypted SQLite database for data persistence
//! - **CLI interface**: Easy-to-use command line interface
//! - **Password generation**: Cryptographically secure password generation
//! - **Clipboard integration**: Copy passwords directly to clipboard
//! - **Cross-platform**: Works on Linux, macOS, and Windows

pub mod cli;
pub mod config;
pub mod crypto;
pub mod database;
pub mod error;
pub mod utils;

#[cfg(feature = "web-ui")]
pub mod web;

pub use error::{Error, Result};

/// Application name constant
pub const APP_NAME: &str = "passman-cli";

/// Default database file name
pub const DEFAULT_DB_NAME: &str = "passman.db";

/// Configuration file name
pub const CONFIG_FILE_NAME: &str = "config.toml";
