use crate::{Error, Result};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{rand_core::OsRng, SaltString};
use ring::rand::{SecureRandom, SystemRandom};
use zeroize::Zeroize;

/// Password hashing and verification utilities
pub struct PasswordManager {
    argon2: Argon2<'static>,
    rng: SystemRandom,
}

impl PasswordManager {
    /// Create a new password manager
    pub fn new() -> Self {
        Self {
            argon2: Argon2::default(),
            rng: SystemRandom::new(),
        }
    }

    /// Hash a master password with a salt
    pub fn hash_password(&self, password: &str) -> Result<(String, Vec<u8>)> {
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = self.argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(Error::from)?
            .to_string();
        
        Ok((password_hash, salt.as_str().as_bytes().to_vec()))
    }

    /// Verify a password against a hash
    pub fn verify_password(&self, password: &str, hash: &str) -> Result<bool> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| Error::Crypto(format!("Invalid hash format: {}", e)))?;
        
        match self.argon2.verify_password(password.as_bytes(), &parsed_hash) {
            Ok(()) => Ok(true),
            Err(argon2::password_hash::Error::Password) => Ok(false),
            Err(e) => Err(Error::from(e)),
        }
    }

    /// Generate a random salt
    pub fn generate_salt(&self) -> Result<Vec<u8>> {
        let mut salt = vec![0u8; 32];
        self.rng.fill(&mut salt)
            .map_err(|_| Error::Crypto("Failed to generate salt".to_string()))?;
        Ok(salt)
    }

    /// Derive an encryption key from a password and salt
    pub fn derive_key(&self, password: &str, salt: &[u8]) -> Result<Vec<u8>> {
        let mut key = vec![0u8; 32]; // 256-bit key
        
        // Use Argon2 for key derivation
        let salt_string = SaltString::encode_b64(salt)
            .map_err(|e| Error::Crypto(format!("Invalid salt: {}", e)))?;
        
        let password_hash = self.argon2
            .hash_password(password.as_bytes(), &salt_string)
            .map_err(Error::from)?;
        
        // Extract the hash bytes (32 bytes for our key)
        let hash = password_hash.hash.unwrap();
        let hash_bytes = hash.as_bytes();
        let copy_len = std::cmp::min(key.len(), hash_bytes.len());
        key[..copy_len].copy_from_slice(&hash_bytes[..copy_len]);
        
        Ok(key)
    }
}

impl Default for PasswordManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Secure password input utility
pub fn read_password(prompt: &str) -> Result<String> {
    let password = rpassword::prompt_password(prompt)
        .map_err(|e| Error::Io(e))?;
    
    if password.trim().is_empty() {
        return Err(Error::InvalidInput("Password cannot be empty".to_string()));
    }
    
    Ok(password)
}

/// Secure password confirmation
pub fn read_password_with_confirmation(prompt: &str) -> Result<String> {
    let password = read_password(prompt)?;
    let confirm = read_password("Confirm password: ")?;
    
    if password != confirm {
        // Zero out the passwords
        let mut pwd = password;
        let mut conf = confirm;
        pwd.zeroize();
        conf.zeroize();
        
        return Err(Error::InvalidInput("Passwords do not match".to_string()));
    }
    
    Ok(password)
}
