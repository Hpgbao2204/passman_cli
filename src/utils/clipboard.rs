use crate::{Error, Result};
use std::time::Duration;

/// Clipboard manager for secure password copying
pub struct ClipboardManager {
    timeout: Duration,
}

impl ClipboardManager {
    /// Create a new clipboard manager with timeout
    pub fn new(timeout_seconds: u64) -> Self {
        Self {
            timeout: Duration::from_secs(timeout_seconds),
        }
    }

    /// Copy text to clipboard
    pub fn copy(&self, text: &str) -> Result<()> {
        use clipboard::{ClipboardContext, ClipboardProvider};
        
        let mut ctx: ClipboardContext = ClipboardProvider::new()
            .map_err(|e| Error::Clipboard(format!("Failed to access clipboard: {}", e)))?;
        
        ctx.set_contents(text.to_string())
            .map_err(|e| Error::Clipboard(format!("Failed to copy to clipboard: {}", e)))?;
        
        Ok(())
    }

    /// Copy text to clipboard with auto-clear
    pub fn copy_with_timeout(&self, text: &str) -> Result<()> {
        self.copy(text)?;
        
        if self.timeout.as_secs() > 0 {
            println!("Password copied to clipboard (will be cleared in {} seconds)", 
                     self.timeout.as_secs());
            
            // Spawn a thread to clear clipboard after timeout
            let timeout = self.timeout;
            
            std::thread::spawn(move || {
                std::thread::sleep(timeout);
                // Simply clear the clipboard after timeout
                let _ = Self::clear_clipboard();
            });
        } else {
            println!("Password copied to clipboard");
        }
        
        Ok(())
    }

    /// Get current clipboard contents
    pub fn get(&self) -> Result<String> {
        use clipboard::{ClipboardContext, ClipboardProvider};
        
        let mut ctx: ClipboardContext = ClipboardProvider::new()
            .map_err(|e| Error::Clipboard(format!("Failed to access clipboard: {}", e)))?;
        
        ctx.get_contents()
            .map_err(|e| Error::Clipboard(format!("Failed to read from clipboard: {}", e)))
    }

    /// Clear clipboard
    pub fn clear(&self) -> Result<()> {
        self.copy("")
    }

    /// Clear clipboard (static method for thread use)
    fn clear_clipboard() -> Result<()> {
        use clipboard::{ClipboardContext, ClipboardProvider};
        let mut ctx: ClipboardContext = ClipboardProvider::new()
            .map_err(|e| Error::Clipboard(format!("Failed to access clipboard: {}", e)))?;
        ctx.set_contents(String::new())
            .map_err(|e| Error::Clipboard(format!("Failed to clear clipboard: {}", e)))?;
        Ok(())
    }
}

impl Default for ClipboardManager {
    fn default() -> Self {
        Self::new(30) // 30 seconds default timeout
    }
}

/// Copy password to clipboard with default settings
pub fn copy_password(password: &str) -> Result<()> {
    let manager = ClipboardManager::default();
    manager.copy_with_timeout(password)
}

/// Copy text to clipboard without timeout
pub fn copy_text(text: &str) -> Result<()> {
    let manager = ClipboardManager::new(0);
    manager.copy(text)
}
