use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "passman")]
#[command(author = "Hpgbao2204")]
#[command(version = "0.1.0")]
#[command(about = "A secure offline password manager CLI tool")]
#[command(long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new password vault
    Init {
        /// Force initialization even if vault exists
        #[arg(short, long)]
        force: bool,
    },
    /// Add a new password entry
    Add {
        /// Name/title of the entry
        name: String,
        /// Website URL (optional)
        #[arg(short, long)]
        url: Option<String>,
        /// Additional notes (optional)
        #[arg(short, long)]
        notes: Option<String>,
    },
    /// Get a password entry
    Get {
        /// Name/title of the entry to retrieve
        name: String,
    },
    /// List all password entries
    List,
    /// Edit an existing password entry
    Edit {
        /// Name/title of the entry to edit
        name: String,
    },
    /// Delete a password entry
    Delete {
        /// Name/title of the entry to delete
        name: String,
        /// Skip confirmation prompt
        #[arg(short, long)]
        force: bool,
    },
    /// Generate a secure password
    Generate {
        /// Password length (default: 16)
        #[arg(short, long, default_value_t = 16)]
        length: u32,
        /// Exclude symbols from generated password
        #[arg(long)]
        no_symbols: bool,
        /// Exclude numbers from generated password
        #[arg(long)]
        no_numbers: bool,
    },
    /// Copy password to clipboard
    Copy {
        /// Name/title of the entry to copy
        name: String,
    },
    /// Search password entries
    Search {
        /// Search query
        query: String,
    },
    /// Start web interface
    #[cfg(feature = "web-ui")]
    Web {
        /// Port to run web server on
        #[arg(short, long, default_value_t = 8080)]
        port: u16,
    },
}
