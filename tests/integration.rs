use smartpasslib::*;

#[test]
fn test_smart_password_consistency() {
    let secret = "MyStrongSecretPhrase2026!";
    let len = 16;
    let password = generate_smart_password_sync(secret, len).unwrap();

    assert_eq!(password.len(), len);

    for c in password.chars() {
        assert!(CHARS.contains(c));
    }
}

#[test]
fn test_private_key_consistency() {
    let secret = "MyStrongSecretPhrase2026!";
    let key1 = generate_private_key(secret).unwrap();
    let key2 = generate_private_key(secret).unwrap();
    assert_eq!(key1, key2);
}

#[test]
fn test_public_key_consistency() {
    let secret = "MyStrongSecretPhrase2026!";
    let key1 = generate_public_key(secret).unwrap();
    let key2 = generate_public_key(secret).unwrap();
    assert_eq!(key1, key2);
}

#[test]
fn test_verify_secret() {
    let secret = "MyStrongSecretPhrase2026!";
    let public_key = generate_public_key(secret).unwrap();
    assert!(verify_secret(secret, &public_key).unwrap());
    assert!(!verify_secret("WrongSecret123!", &public_key).unwrap());
}

#[test]
fn test_secret_too_short() {
    let secret = "short";
    assert!(generate_private_key(secret).is_err());
    assert!(generate_public_key(secret).is_err());
    assert!(generate_smart_password_sync(secret, 16).is_err());
}

#[test]
fn test_password_length_validation() {
    let secret = "MyStrongSecretPhrase2026!";
    assert!(generate_smart_password_sync(secret, 11).is_err());
    assert!(generate_smart_password_sync(secret, 101).is_err());
    assert!(generate_smart_password_sync(secret, 12).is_ok());
    assert!(generate_smart_password_sync(secret, 100).is_ok());
}

#[test]
fn test_code_length_validation() {
    assert!(generate_code(3).is_err());
    assert!(generate_code(101).is_err());
    assert!(generate_code(4).is_ok());
    assert!(generate_code(100).is_ok());
}

#[test]
fn test_strong_password() {
    let p1 = generate_strong_password(20).unwrap();
    let p2 = generate_strong_password(20).unwrap();
    assert_eq!(p1.len(), 20);
    assert_eq!(p2.len(), 20);
    for c in p1.chars() {
        assert!(CHARS.contains(c));
    }
}

#[test]
fn test_chars_set() {
    let chars: String = CHARS.chars().collect();
    assert!(chars.contains('!'));
    assert!(chars.contains('@'));
    assert!(chars.contains('#'));
    assert!(chars.contains('A'));
    assert!(chars.contains('Z'));
    assert!(chars.contains('0'));
    assert!(chars.contains('9'));
    assert!(chars.contains('a'));
    assert!(chars.contains('z'));
    assert!(!chars.is_empty());
}

#[test]
fn test_manager_operations() {
    use tempfile::tempdir;

    let dir = tempdir().unwrap();
    let file = dir.path().join("test_passwords.json");

    let mut manager = SmartPasswordManager::with_filename(&file).unwrap();
    assert!(manager.is_empty());

    let public_key = "test_key_123".to_string();
    let sp = SmartPassword::new(public_key.clone(), "Test Service".to_string(), 16).unwrap();

    manager.add(sp).unwrap();
    assert_eq!(manager.len(), 1);

    let retrieved = manager.get(&public_key).unwrap();
    assert_eq!(retrieved.description, "Test Service");

    let deleted = manager.delete(&public_key).unwrap();
    assert!(deleted);
    assert_eq!(manager.len(), 0);
    assert!(manager.is_empty());
}
