use crate::{Error, Result};
use rand::thread_rng;
use rand::seq::SliceRandom;

/// Password generation configuration
#[derive(Debug, Clone)]
pub struct GeneratorConfig {
    pub length: u32,
    pub include_uppercase: bool,
    pub include_lowercase: bool,
    pub include_numbers: bool,
    pub include_symbols: bool,
    pub symbol_set: String,
}

impl Default for GeneratorConfig {
    fn default() -> Self {
        Self {
            length: 16,
            include_uppercase: true,
            include_lowercase: true,
            include_numbers: true,
            include_symbols: true,
            symbol_set: "!@#$%^&*()-_=+[]{}|;:,.<>?".to_string(),
        }
    }
}

/// Password generator
pub struct PasswordGenerator {
    config: GeneratorConfig,
}

impl PasswordGenerator {
    /// Create a new password generator with default config
    pub fn new() -> Self {
        Self {
            config: GeneratorConfig::default(),
        }
    }

    /// Create a password generator with custom config
    pub fn with_config(config: GeneratorConfig) -> Self {
        Self { config }
    }

    /// Generate a password
    pub fn generate(&self) -> Result<String> {
        if self.config.length == 0 {
            return Err(Error::PasswordGeneration("Password length cannot be zero".to_string()));
        }

        let mut charset = String::new();
        
        if self.config.include_lowercase {
            charset.push_str("abcdefghijklmnopqrstuvwxyz");
        }
        
        if self.config.include_uppercase {
            charset.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        }
        
        if self.config.include_numbers {
            charset.push_str("0123456789");
        }
        
        if self.config.include_symbols {
            charset.push_str(&self.config.symbol_set);
        }

        if charset.is_empty() {
            return Err(Error::PasswordGeneration("No character sets selected".to_string()));
        }

        let charset_chars: Vec<char> = charset.chars().collect();
        let mut rng = thread_rng();
        let mut password = String::new();

        // Ensure at least one character from each enabled set
        if self.config.include_lowercase {
            let lowercase: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
            password.push(*lowercase.choose(&mut rng).unwrap());
        }
        
        if self.config.include_uppercase {
            let uppercase: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
            password.push(*uppercase.choose(&mut rng).unwrap());
        }
        
        if self.config.include_numbers {
            let numbers: Vec<char> = "0123456789".chars().collect();
            password.push(*numbers.choose(&mut rng).unwrap());
        }
        
        if self.config.include_symbols {
            let symbols: Vec<char> = self.config.symbol_set.chars().collect();
            password.push(*symbols.choose(&mut rng).unwrap());
        }

        // Fill the rest randomly
        while password.len() < self.config.length as usize {
            let random_char = charset_chars.choose(&mut rng).unwrap();
            password.push(*random_char);
        }

        // Shuffle the password to avoid predictable patterns
        let mut password_chars: Vec<char> = password.chars().collect();
        password_chars.shuffle(&mut rng);
        
        Ok(password_chars.into_iter().collect())
    }

    /// Generate multiple passwords
    pub fn generate_batch(&self, count: u32) -> Result<Vec<String>> {
        let mut passwords = Vec::with_capacity(count as usize);
        for _ in 0..count {
            passwords.push(self.generate()?);
        }
        Ok(passwords)
    }

    /// Set password length
    pub fn set_length(&mut self, length: u32) {
        self.config.length = length;
    }

    /// Enable/disable character sets
    pub fn set_include_uppercase(&mut self, include: bool) {
        self.config.include_uppercase = include;
    }

    pub fn set_include_lowercase(&mut self, include: bool) {
        self.config.include_lowercase = include;
    }

    pub fn set_include_numbers(&mut self, include: bool) {
        self.config.include_numbers = include;
    }

    pub fn set_include_symbols(&mut self, include: bool) {
        self.config.include_symbols = include;
    }

    /// Set custom symbol set
    pub fn set_symbol_set(&mut self, symbols: String) {
        self.config.symbol_set = symbols;
    }
}

impl Default for PasswordGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Generate a simple password with default settings
pub fn generate_password(length: u32) -> Result<String> {
    let mut config = GeneratorConfig::default();
    config.length = length;
    
    let generator = PasswordGenerator::with_config(config);
    generator.generate()
}

/// Generate a password with no symbols
pub fn generate_alphanumeric_password(length: u32) -> Result<String> {
    let config = GeneratorConfig {
        length,
        include_uppercase: true,
        include_lowercase: true,
        include_numbers: true,
        include_symbols: false,
        symbol_set: String::new(),
    };
    
    let generator = PasswordGenerator::with_config(config);
    generator.generate()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_generation() {
        let generator = PasswordGenerator::new();
        let password = generator.generate().unwrap();
        assert_eq!(password.len(), 16);
    }

    #[test]
    fn test_custom_length() {
        let mut config = GeneratorConfig::default();
        config.length = 32;
        
        let generator = PasswordGenerator::with_config(config);
        let password = generator.generate().unwrap();
        assert_eq!(password.len(), 32);
    }

    #[test]
    fn test_alphanumeric_only() {
        let password = generate_alphanumeric_password(20).unwrap();
        assert_eq!(password.len(), 20);
        assert!(password.chars().all(|c| c.is_alphanumeric()));
    }
}
