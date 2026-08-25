//! SmartPassLib — cryptographic password generation without storage.
//!
//! Cross-platform deterministic password generation compatible with Python, JavaScript,
//! Kotlin, Go, and C# implementations.
//!
//! # Example
//! ```
//! use smartpasslib::*;
//!
//! let secret = "MyStrongSecretPhrase2026!";
//! let password = generate_smart_password_sync(secret, 16).unwrap();
//! println!("{}", password);
//! ```

pub mod core;
pub mod error;
pub mod manager;

pub use core::{
    CHARS, generate_base_password, generate_code, generate_private_key, generate_public_key,
    generate_smart_password_sync, generate_strong_password, verify_secret,
};

pub use manager::{SmartPassword, SmartPasswordManager};

pub use error::{Result, SmartPassError};

pub const VERSION: &str = "4.0.0";
