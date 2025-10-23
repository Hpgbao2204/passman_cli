# PassMan-CLI

A secure offline password manager CLI tool built with Rust.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://github.com/Hpgbao2204/passman_cli/workflows/Rust/badge.svg)](https://github.com/Hpgbao2204/passman_cli/actions)

## 🔐 Features

- **🔒 Offline-first**: No cloud dependencies, all data stored locally
- **🛡️ Secure encryption**: Uses AES-256-GCM with Argon2 key derivation
- **💾 SQLCipher**: Encrypted SQLite database for data persistence
- **⌨️ CLI interface**: Easy-to-use command line interface
- **🎲 Password generation**: Cryptographically secure password generation
- **📋 Clipboard integration**: Copy passwords directly to clipboard
- **🔍 Search functionality**: Find entries quickly
- **🌐 Cross-platform**: Works on Linux, macOS, and Windows

## 🚀 Quick Start

### Installation

```bash
# Clone the repository
git clone https://github.com/Hpgbao2204/passman_cli.git
cd passman_cli

# Build and install
cargo install --path .
```

### Basic Usage

```bash
# Initialize a new vault
passman init

# Add a new password entry
passman add "GitHub" --url "https://github.com"

# List all entries
passman list

# Get a password entry
passman get "GitHub"

# Copy password to clipboard
passman copy "GitHub"

# Generate a secure password
passman generate --length 20

# Search entries
passman search "git"

# Edit an entry
passman edit "GitHub"

# Delete an entry
passman delete "GitHub"
```

## 📖 Commands

### `passman init [--force]`
Initialize a new password vault. Use `--force` to reinitialize an existing vault.

### `passman add <name> [--url <url>] [--notes <notes>]`
Add a new password entry. You'll be prompted to enter username and password.

### `passman get <name>`
Display information for a password entry (password will be hidden by default).

### `passman list`
List all password entries with their titles and usernames.

### `passman edit <name>`
Edit an existing password entry.

### `passman delete <name> [--force]`
Delete a password entry. Use `--force` to skip confirmation.

### `passman copy <name>`
Copy the password for an entry to your clipboard.

### `passman generate [options]`
Generate a secure password with customizable options:
- `--length <n>`: Password length (default: 16)
- `--no-symbols`: Exclude symbols
- `--no-numbers`: Exclude numbers

### `passman search <query>`
Search for entries by name, username, URL, or notes.

## 🔧 Configuration

PassMan-CLI uses a configuration file located at:
- Linux/macOS: `~/.config/passman-cli/config.toml`
- Windows: `%APPDATA%\passman-cli\config.toml`

Example configuration:
```toml
[password_generation]
default_length = 16
include_uppercase = true
include_lowercase = true
include_numbers = true
include_symbols = true
symbol_set = "!@#$%^&*()-_=+[]{}|;:,.<>?"

[security]
session_timeout = 15  # minutes
max_login_attempts = 3
lockout_duration = 5  # minutes

clipboard_timeout = 30  # seconds
```

## 🔒 Security

### Encryption
- **Master password**: Protected with Argon2 password hashing
- **Data encryption**: AES-256-GCM for all sensitive data
- **Key derivation**: PBKDF2 with salt for encryption keys
- **Database**: SQLCipher for encrypted SQLite storage

### Memory Safety
- Sensitive data is zeroed from memory after use
- Secure string types prevent accidental data leaks
- No sensitive data in swap files or core dumps

### Best Practices
- Use a strong, unique master password
- Keep your vault file backed up securely
- Regularly update your passwords
- Don't share your master password

## 🧪 Development

### Prerequisites
- Rust 1.70.0 or higher
- SQLite development libraries

### Building from Source
```bash
# Clone the repository
git clone https://github.com/Hpgbao2204/passman_cli.git
cd passman_cli

# Build in debug mode
cargo build

# Build in release mode
cargo build --release

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run
```

### Project Structure
```
src/
├── main.rs              # Entry point
├── lib.rs               # Library root
├── error.rs             # Error types
├── cli/                 # CLI interface
├── crypto/              # Encryption & security
├── database/            # Database operations
├── config/              # Configuration
└── utils/               # Utilities
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the project
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## ⚠️ Disclaimer

This software is provided "as is" without warranty of any kind. Use at your own risk. Always backup your password vault and remember your master password.

## 🙏 Acknowledgments

- Inspired by [pass](https://www.passwordstore.org/) and [Bitwarden CLI](https://bitwarden.com/help/cli/)
- Built with amazing Rust libraries: clap, rusqlite, ring, argon2, and more