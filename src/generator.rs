//! Core password generation logic.

use rand::RngCore;

use crate::core::{CHARS, hex_to_bytes, sha256_hex};
use crate::error::SmartPassError;

fn validate_secret(secret: &str) -> Result<(), SmartPassError> {
    if secret.len() < 12 {
        return Err(SmartPassError::SecretTooShort(secret.len()));
    }
    Ok(())
}

fn validate_password_length(length: usize) -> Result<(), SmartPassError> {
    if !(12..=100).contains(&length) {
        return Err(SmartPassError::InvalidPasswordLength(length));
    }
    Ok(())
}

fn validate_code_length(length: usize) -> Result<(), SmartPassError> {
    if !(4..=100).contains(&length) {
        return Err(SmartPassError::InvalidCodeLength(length));
    }
    Ok(())
}

fn get_steps_from_secret(secret: &str, min: usize, max: usize, salt: &str) -> usize {
    let hash = sha256_hex(&format!("{}:{}", secret, salt));
    let hash_int = u64::from_str_radix(&hash[..8], 16).unwrap_or(0);
    min + (hash_int as usize % (max - min + 1))
}

fn derive_key(secret: &str, steps: usize, salt: &str) -> String {
    let mut hash = sha256_hex(&format!("{}:{}", secret, salt));
    for i in 0..steps {
        hash = sha256_hex(&format!("{}:{}", hash, i));
    }
    hash
}

fn generate_password_from_private_key(private_key: &str, length: usize) -> String {
    let mut result = String::with_capacity(length);
    let mut counter = 0;

    while result.len() < length {
        let hash = sha256_hex(&format!("{}:{}", private_key, counter));
        let bytes = hex_to_bytes(&hash);
        let remaining = length - result.len();
        let to_take = bytes.len().min(remaining);

        for &byte in &bytes[..to_take] {
            let idx = (byte as usize) % CHARS.len();
            result.push(CHARS.chars().nth(idx).unwrap());
        }
        counter += 1;
    }

    result
}

pub fn generate_private_key(secret: &str) -> Result<String, SmartPassError> {
    validate_secret(secret)?;
    let steps = get_steps_from_secret(secret, 15, 30, "private");
    Ok(derive_key(secret, steps, "private"))
}

pub fn generate_public_key(secret: &str) -> Result<String, SmartPassError> {
    validate_secret(secret)?;
    let steps = get_steps_from_secret(secret, 45, 60, "public");
    Ok(derive_key(secret, steps, "public"))
}

pub fn verify_secret(secret: &str, public_key: &str) -> Result<bool, SmartPassError> {
    let computed = generate_public_key(secret)?;
    Ok(computed == public_key)
}

pub fn generate_smart_password(secret: &str, length: usize) -> Result<String, SmartPassError> {
    validate_secret(secret)?;
    validate_password_length(length)?;
    let private_key = generate_private_key(secret)?;
    Ok(generate_password_from_private_key(&private_key, length))
}

pub fn generate_strong_password(length: usize) -> Result<String, SmartPassError> {
    validate_password_length(length)?;
    let mut bytes = vec![0u8; length];
    rand::thread_rng().fill_bytes(&mut bytes);
    Ok(bytes
        .iter()
        .map(|&b| CHARS.chars().nth((b as usize) % CHARS.len()).unwrap())
        .collect())
}

pub fn generate_base_password(length: usize) -> Result<String, SmartPassError> {
    generate_strong_password(length)
}

pub fn generate_code(length: usize) -> Result<String, SmartPassError> {
    validate_code_length(length)?;
    let mut bytes = vec![0u8; length];
    rand::thread_rng().fill_bytes(&mut bytes);
    Ok(bytes
        .iter()
        .map(|&b| CHARS.chars().nth((b as usize) % CHARS.len()).unwrap())
        .collect())
}
