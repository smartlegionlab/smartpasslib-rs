//! Error types for SmartPassLib.

use thiserror::Error;

/// Errors that can occur in SmartPassLib.
#[derive(Error, Debug)]
pub enum SmartPassError {
    /// Secret phrase is too short (minimum 12 characters).
    #[error("Secret phrase must be at least 12 characters. Current: {0}")]
    SecretTooShort(usize),

    /// Password length is outside valid range (12-100).
    #[error("Password length must be between {min} and {max}. Current: {current}")]
    InvalidPasswordLength {
        min: usize,
        max: usize,
        current: usize,
    },

    /// Code length is outside valid range (4-100).
    #[error("Code length must be between 4 and 100. Current: {0}")]
    InvalidCodeLength(usize),

    /// Public key cannot be empty.
    #[error("Public key cannot be empty")]
    EmptyPublicKey,

    /// Description cannot be empty.
    #[error("Description cannot be empty")]
    EmptyDescription,

    /// Cryptographic error occurred.
    #[error("Crypto error: {0}")]
    CryptoError(String),

    /// IO error occurred.
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// Serialization error occurred.
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    /// Hex decoding error occurred.
    #[error("Hex error: {0}")]
    HexError(#[from] hex::FromHexError),
}

/// Result type for SmartPassLib.
pub type Result<T> = std::result::Result<T, SmartPassError>;
