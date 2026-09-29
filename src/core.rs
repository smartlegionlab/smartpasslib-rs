//! Core password generation functionality.

use crate::error::{Result, SmartPassError};
use rand::Rng;
use sha2::{Digest, Sha256};

/// Character set for password generation.
///
/// Must match exactly with Python, JavaScript, Kotlin, Go, and C# implementations:
/// symbols + uppercase + digits + lowercase
pub const CHARS: &str =
    "!@#$%^&*()_+-=[]{};:,.<>?/ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789abcdefghijklmnopqrstuvwxyz";

const MIN_SECRET_LEN: usize = 12;
const MIN_PASSWORD_LEN: usize = 12;
const MAX_PASSWORD_LEN: usize = 100;
const MIN_CODE_LEN: usize = 4;
const MAX_CODE_LEN: usize = 100;

/// SHA-256 hash function (same as other implementations).
fn sha256_hash(text: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(text.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}

/// Validate secret phrase length.
fn validate_secret(secret: &str) -> Result<()> {
    if secret.len() < MIN_SECRET_LEN {
        return Err(SmartPassError::SecretTooShort(secret.len()));
    }
    Ok(())
}

/// Validate password length.
fn validate_password_length(length: usize) -> Result<()> {
    if length < MIN_PASSWORD_LEN {
        return Err(SmartPassError::InvalidPasswordLength {
            min: MIN_PASSWORD_LEN,
            max: MAX_PASSWORD_LEN,
            current: length,
        });
    }
    if length > MAX_PASSWORD_LEN {
        return Err(SmartPassError::InvalidPasswordLength {
            min: MIN_PASSWORD_LEN,
            max: MAX_PASSWORD_LEN,
            current: length,
        });
    }
    Ok(())
}

/// Validate code length.
fn validate_code_length(length: usize) -> Result<()> {
    if length < MIN_CODE_LEN {
        return Err(SmartPassError::InvalidCodeLength(length));
    }
    if length > MAX_CODE_LEN {
        return Err(SmartPassError::InvalidCodeLength(length));
    }
    Ok(())
}

/// Get deterministic steps count from secret hash.
fn get_steps_from_secret(
    secret: &str,
    min_steps: usize,
    max_steps: usize,
    salt: &str,
) -> Result<usize> {
    let hash_value = sha256_hash(&format!("{}:{}", secret, salt));
    let hash_int = u64::from_str_radix(&hash_value[0..8], 16)
        .map_err(|_| SmartPassError::CryptoError("Failed to parse hash".to_string()))?;
    let steps = min_steps + (hash_int as usize % (max_steps - min_steps + 1));
    Ok(steps)
}

/// Generate a key from secret with specified iterations.
fn generate_key(secret: &str, steps: usize, salt: &str) -> Result<String> {
    validate_secret(secret)?;

    let mut hash = sha256_hash(&format!("{}:{}", secret, salt));

    for i in 0..steps {
        hash = sha256_hash(&format!("{}:{}", hash, i));
    }

    Ok(hash)
}

/// Generate private key (15-30 iterations).
pub fn generate_private_key(secret: &str) -> Result<String> {
    validate_secret(secret)?;
    let steps = get_steps_from_secret(secret, 15, 30, "private")?;
    generate_key(secret, steps, "private")
}

/// Generate public key (45-60 iterations).
pub fn generate_public_key(secret: &str) -> Result<String> {
    validate_secret(secret)?;
    let steps = get_steps_from_secret(secret, 45, 60, "public")?;
    generate_key(secret, steps, "public")
}

/// Verify that a secret matches a public key.
pub fn verify_secret(secret: &str, public_key: &str) -> Result<bool> {
    let computed = generate_public_key(secret)?;
    Ok(computed == public_key)
}

/// Convert hex string to bytes.
fn hex_to_bytes(hex_str: &str) -> Result<Vec<u8>> {
    hex::decode(hex_str).map_err(SmartPassError::from)
}

/// Generate password from private key.
fn generate_password_from_private_key(private_key: &str, length: usize) -> Result<String> {
    validate_password_length(length)?;

    let mut result = String::with_capacity(length);
    let mut counter = 0;
    let chars = CHARS.as_bytes();

    while result.len() < length {
        let hash_hex = sha256_hash(&format!("{}:{}", private_key, counter));
        let hash_bytes = hex_to_bytes(&hash_hex)?;

        for &byte in &hash_bytes {
            if result.len() >= length {
                break;
            }
            let idx = (byte as usize) % chars.len();
            result.push(chars[idx] as char);
        }
        counter += 1;
    }

    Ok(result)
}

/// Generate deterministic smart password from secret.
pub fn generate_smart_password_sync(secret: &str, length: usize) -> Result<String> {
    validate_secret(secret)?;
    validate_password_length(length)?;
    let private_key = generate_private_key(secret)?;
    generate_password_from_private_key(&private_key, length)
}

/// Generate cryptographically secure random password.
pub fn generate_strong_password(length: usize) -> Result<String> {
    validate_password_length(length)?;

    let mut rng = rand::rng();
    let chars = CHARS.as_bytes();
    let mut result = String::with_capacity(length);

    for _ in 0..length {
        let idx = rng.random_range(0..chars.len());
        result.push(chars[idx] as char);
    }

    Ok(result)
}

/// Generate base random password.
pub fn generate_base_password(length: usize) -> Result<String> {
    generate_strong_password(length)
}

/// Generate authentication code (for 2FA).
pub fn generate_code(length: usize) -> Result<String> {
    validate_code_length(length)?;

    let mut rng = rand::rng();
    let chars = CHARS.as_bytes();
    let mut result = String::with_capacity(length);

    for _ in 0..length {
        let idx = rng.random_range(0..chars.len());
        result.push(chars[idx] as char);
    }

    Ok(result)
}
