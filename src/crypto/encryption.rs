use crate::{Error, Result};
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305, Key, Nonce,
};
use ring::rand::{SecureRandom, SystemRandom};
use zeroize::Zeroize;

/// Encryption manager using ChaCha20Poly1305
pub struct EncryptionManager {
    rng: SystemRandom,
}

impl EncryptionManager {
    /// Create a new encryption manager
    pub fn new() -> Self {
        Self {
            rng: SystemRandom::new(),
        }
    }

    /// Encrypt data with a given key
    pub fn encrypt(&self, key: &[u8], plaintext: &[u8]) -> Result<Vec<u8>> {
        if key.len() != 32 {
            return Err(Error::Crypto("Key must be 32 bytes".to_string()));
        }

        let key = Key::from_slice(key);
        let cipher = ChaCha20Poly1305::new(key);
        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
        
        let ciphertext = cipher
            .encrypt(&nonce, plaintext)
            .map_err(|e| Error::Crypto(format!("Encryption failed: {}", e)))?;
        
        // Prepend nonce to ciphertext
        let mut result = nonce.to_vec();
        result.extend_from_slice(&ciphertext);
        
        Ok(result)
    }

    /// Decrypt data with a given key
    pub fn decrypt(&self, key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>> {
        if key.len() != 32 {
            return Err(Error::Crypto("Key must be 32 bytes".to_string()));
        }

        if ciphertext.len() < 12 {
            return Err(Error::Crypto("Ciphertext too short".to_string()));
        }

        let key = Key::from_slice(key);
        let cipher = ChaCha20Poly1305::new(key);
        
        // Extract nonce and ciphertext
        let (nonce_bytes, encrypted_data) = ciphertext.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);
        
        let plaintext = cipher
            .decrypt(nonce, encrypted_data)
            .map_err(|e| Error::Crypto(format!("Decryption failed: {}", e)))?;
        
        Ok(plaintext)
    }

    /// Generate a random encryption key
    pub fn generate_key(&self) -> Result<Vec<u8>> {
        let mut key = vec![0u8; 32];
        self.rng.fill(&mut key)
            .map_err(|_| Error::Crypto("Failed to generate key".to_string()))?;
        Ok(key)
    }

    /// Generate random bytes
    pub fn generate_random(&self, length: usize) -> Result<Vec<u8>> {
        let mut bytes = vec![0u8; length];
        self.rng.fill(&mut bytes)
            .map_err(|_| Error::Crypto("Failed to generate random bytes".to_string()))?;
        Ok(bytes)
    }
}

impl Default for EncryptionManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Secure data wrapper that encrypts on creation and decrypts on access
pub struct SecureData {
    encrypted_data: Vec<u8>,
    manager: EncryptionManager,
}

impl SecureData {
    /// Create secure data from plaintext and key
    pub fn new(plaintext: &[u8], key: &[u8]) -> Result<Self> {
        let manager = EncryptionManager::new();
        let encrypted_data = manager.encrypt(key, plaintext)?;
        
        Ok(Self {
            encrypted_data,
            manager,
        })
    }

    /// Decrypt and return the data
    pub fn decrypt(&self, key: &[u8]) -> Result<Vec<u8>> {
        self.manager.decrypt(key, &self.encrypted_data)
    }

    /// Get the encrypted data
    pub fn encrypted_bytes(&self) -> &[u8] {
        &self.encrypted_data
    }
}

impl Drop for SecureData {
    fn drop(&mut self) {
        self.encrypted_data.zeroize();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption_decryption() {
        let manager = EncryptionManager::new();
        let key = manager.generate_key().unwrap();
        let plaintext = b"Hello, World!";
        
        let ciphertext = manager.encrypt(&key, plaintext).unwrap();
        let decrypted = manager.decrypt(&key, &ciphertext).unwrap();
        
        assert_eq!(plaintext, &decrypted[..]);
    }

    #[test]
    fn test_secure_data() {
        let key = EncryptionManager::new().generate_key().unwrap();
        let plaintext = b"Secret data";
        
        let secure_data = SecureData::new(plaintext, &key).unwrap();
        let decrypted = secure_data.decrypt(&key).unwrap();
        
        assert_eq!(plaintext, &decrypted[..]);
    }
}
