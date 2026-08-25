//! Password metadata storage manager.

use crate::error::{Result, SmartPassError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Metadata container for a smart password.
///
/// Stores only public verification data, never the secret or actual password.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartPassword {
    /// Public verification key (derived from secret).
    pub public_key: String,
    /// Service/account description.
    pub description: String,
    /// Password length.
    pub length: usize,
}

impl SmartPassword {
    /// Create new smart password metadata.
    pub fn new(public_key: String, description: String, length: usize) -> Result<Self> {
        if public_key.is_empty() {
            return Err(SmartPassError::EmptyPublicKey);
        }
        if description.is_empty() {
            return Err(SmartPassError::EmptyDescription);
        }
        if !(12..=100).contains(&length) {
            return Err(SmartPassError::InvalidPasswordLength {
                min: 12,
                max: 100,
                current: length,
            });
        }

        Ok(Self {
            public_key,
            description,
            length,
        })
    }

    /// Update metadata fields.
    pub fn update(&mut self, description: Option<&str>, length: Option<usize>) -> Result<()> {
        if let Some(desc) = description {
            self.description = desc.to_string();
        }
        if let Some(len) = length {
            if !(12..=100).contains(&len) {
                return Err(SmartPassError::InvalidPasswordLength {
                    min: 12,
                    max: 100,
                    current: len,
                });
            }
            self.length = len;
        }
        Ok(())
    }
}

/// Manager for persistent storage of password metadata.
///
/// Stores metadata in JSON format at `~/.config/smart_password_manager/passwords.json`.
pub struct SmartPasswordManager {
    passwords: HashMap<String, SmartPassword>,
    filename: PathBuf,
}

impl SmartPasswordManager {
    /// Create manager with default file location.
    pub fn new() -> Result<Self> {
        let home = std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE"))
            .unwrap_or_else(|_| ".".to_string());

        let config_dir = Path::new(&home)
            .join(".config")
            .join("smart_password_manager");
        let filename = config_dir.join("passwords.json");

        let mut manager = Self {
            passwords: HashMap::new(),
            filename,
        };
        manager.load_data()?;
        Ok(manager)
    }

    /// Create manager with custom file location.
    pub fn with_filename<P: AsRef<Path>>(filename: P) -> Result<Self> {
        let mut manager = Self {
            passwords: HashMap::new(),
            filename: filename.as_ref().to_path_buf(),
        };
        manager.load_data()?;
        Ok(manager)
    }

    /// Get all stored passwords.
    pub fn passwords(&self) -> &HashMap<String, SmartPassword> {
        &self.passwords
    }

    /// Number of stored passwords.
    pub fn len(&self) -> usize {
        self.passwords.len()
    }

    /// Check if storage is empty.
    pub fn is_empty(&self) -> bool {
        self.passwords.is_empty()
    }

    /// Get file path.
    pub fn file_path(&self) -> &Path {
        &self.filename
    }

    /// Add a password metadata entry.
    pub fn add(&mut self, password: SmartPassword) -> Result<()> {
        self.passwords.insert(password.public_key.clone(), password);
        self.save_data()?;
        Ok(())
    }

    /// Get password metadata by public key.
    pub fn get(&self, public_key: &str) -> Option<&SmartPassword> {
        self.passwords.get(public_key)
    }

    /// Get mutable password metadata by public key.
    pub fn get_mut(&mut self, public_key: &str) -> Option<&mut SmartPassword> {
        self.passwords.get_mut(public_key)
    }

    /// Update password metadata by public key.
    pub fn update(
        &mut self,
        public_key: &str,
        description: Option<&str>,
        length: Option<usize>,
    ) -> Result<bool> {
        if let Some(password) = self.passwords.get_mut(public_key) {
            password.update(description, length)?;
            self.save_data()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Delete password metadata by public key.
    pub fn delete(&mut self, public_key: &str) -> Result<bool> {
        if self.passwords.remove(public_key).is_some() {
            self.save_data()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Clear all stored password metadata.
    pub fn clear(&mut self) -> Result<()> {
        self.passwords.clear();
        self.save_data()?;
        Ok(())
    }

    /// Load data from file.
    fn load_data(&mut self) -> Result<()> {
        if !self.filename.exists() {
            return Ok(());
        }

        let content = fs::read_to_string(&self.filename)?;

        if content.trim().is_empty() {
            return Ok(());
        }

        let data: HashMap<String, SmartPassword> = serde_json::from_str(&content)?;
        self.passwords = data;
        Ok(())
    }

    /// Save data to file.
    fn save_data(&self) -> Result<()> {
        if let Some(parent) = self.filename.parent()
            && !parent.exists()
        {
            fs::create_dir_all(parent)?;
        }

        let json = serde_json::to_string_pretty(&self.passwords)?;
        fs::write(&self.filename, json)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_smart_password_new() {
        let sp = SmartPassword::new("key".to_string(), "Service".to_string(), 16).unwrap();
        assert_eq!(sp.public_key, "key");
        assert_eq!(sp.description, "Service");
        assert_eq!(sp.length, 16);

        assert!(SmartPassword::new("".to_string(), "desc".to_string(), 12).is_err());
        assert!(SmartPassword::new("key".to_string(), "".to_string(), 12).is_err());
        assert!(SmartPassword::new("key".to_string(), "desc".to_string(), 8).is_err());
    }

    #[test]
    fn test_manager() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("test.json");

        let mut manager = SmartPasswordManager::with_filename(&file).unwrap();
        assert_eq!(manager.len(), 0);

        let sp = SmartPassword::new("key_1".to_string(), "Service 1".to_string(), 16).unwrap();
        manager.add(sp).unwrap();
        assert_eq!(manager.len(), 1);

        let retrieved = manager.get("key_1").unwrap();
        assert_eq!(retrieved.description, "Service 1");

        manager.update("key_1", Some("Updated"), Some(20)).unwrap();
        let updated = manager.get("key_1").unwrap();
        assert_eq!(updated.description, "Updated");
        assert_eq!(updated.length, 20);

        assert!(manager.delete("key_1").unwrap());
        assert_eq!(manager.len(), 0);
    }
}
