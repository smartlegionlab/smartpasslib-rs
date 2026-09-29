# smartpasslib (Rust) <sup>v4.0.0</sup>

[![Crates.io](https://img.shields.io/crates/v/smartpasslib)](https://crates.io/crates/smartpasslib)
[![Documentation](https://docs.rs/smartpasslib/badge.svg)](https://docs.rs/smartpasslib)
[![License](https://img.shields.io/badge/license-BSD--3--Clause-blue.svg)](LICENSE)
[![GitHub stars](https://img.shields.io/github/stars/smartlegionlab/smartpasslib-rs)](https://github.com/smartlegionlab/smartpasslib-rs)
[![GitHub forks](https://img.shields.io/github/forks/smartlegionlab/smartpasslib-rs?style=social)](https://github.com/smartlegionlab/smartpasslib-rs/network/members)
[![Crates.io Downloads](https://img.shields.io/crates/d/smartpasslib)](https://crates.io/crates/smartpasslib)
[![Crates.io Downloads (year)](https://img.shields.io/crates/dy/smartpasslib)](https://crates.io/crates/smartpasslib)
[![Crates.io Downloads (month)](https://img.shields.io/crates/dm/smartpasslib)](https://crates.io/crates/smartpasslib)

**Smart Passwords Library** for Rust: Cryptographic password generation and management without storage. Generate passwords from secrets, verify knowledge without exposure.

**Cross-Platform Determinism**: Same secret + same parameters = identical password on **Rust, C#, Python, Kotlin, Go, JavaScript**.

**Decentralized by Design**: No cloud, no database, no trust required.

---

## ⚠️ Disclaimer

**By using this software, you agree to the full disclaimer terms.**

**Summary:** Software provided "AS IS" without warranty. You assume all risks.

**Full legal disclaimer:** See [DISCLAIMER.md](https://github.com/smartlegionlab/smartpasslib-rs/blob/master/DISCLAIMER.md)

---

## Core Principles

- **Zero-Storage Security**: No passwords or secret phrases are ever stored or transmitted
- **Decentralized Architecture**: No central servers, no cloud dependency
- **Cross-Platform Deterministic Generation**: SHA-256 based, identical across languages
- **Metadata Only**: Store only public keys and descriptions
- **On-Demand Regeneration**: Passwords are recalculated when needed
- **Cryptographically Secure**: Uses SHA-256 and `rand` crate

---

## Key Features

- **Decentralized & Serverless**: No central database, no cloud lock-in
- **Smart Password Generation**: Deterministic from secret phrase
- **Public/Private Key System**: 15-30 iterations for private key, 45-60 for public key (dynamic per secret)
- **Secret Verification**: Verify secret without exposing it
- **Random Password Generation**: Cryptographically secure random passwords
- **Authentication Codes**: Short codes for 2FA/MFA (4-100 chars)
- **Metadata Manager**: Store and manage password metadata locally
- **No External Dependencies**: Pure Rust, uses standard crypto

---

## Security Model

- **Proof of Knowledge**: Public keys verify secrets without exposing them
- **Decentralized Trust**: No third party needed — you control your secrets completely
- **Deterministic Security**: Same input = same output, always reproducible across platforms
- **Dynamic Iteration Counts**: Private key uses 15-30 iterations, public key uses 45-60 iterations (deterministic per secret)
- **No Vulnerable Metadata Storage**: Only public keys and descriptions can be stored (optional)
- **Zero Storage of Secrets**: Secret phrases exist only in your memory, private keys are derived on-demand and never persisted
- **No Recovery Backdoors**: Lost secret = permanently lost passwords (by design)

---

## Research Paradigms & Publications

- **[Pointer-Based Security Paradigm](https://doi.org/10.5281/zenodo.17204738)** - Architectural Shift from Data Protection to Data Non-Existence
- **[Local Data Regeneration Paradigm](https://doi.org/10.5281/zenodo.17264327)** - Ontological Shift from Data Transmission to Synchronous State Discovery

---

## Technical Foundation

**Key derivation (same as Python/JS/Kotlin/Go/C# versions v4.0.0):**

| Key Type | Iterations | Purpose |
|----------|------------|---------|
| Private Key | 15-30 (dynamic) | Password generation (never stored, never transmitted) |
| Public Key | 45-60 (dynamic) | Verification (stored locally) |

**Character Set:** `!@#$%^&*()_+-=[]{};:,.<>?/ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789abcdefghijklmnopqrstuvwxyz`

**Validation Rules:**
- Secret phrase: minimum 12 characters
- Password length: 12-100 characters
- Code length: 4-100 characters

---

## Installation

**Minimum Rust version:** 1.85 (edition 2024)

```toml
[dependencies]
smartpasslib = "4.0"
```

---

## Quick Usage

### Generate Smart Password
```rust
use smartpasslib::generate_smart_password_sync;

let secret = "MyStrongSecretPhrase2026!";
let password = generate_smart_password_sync(secret, 16).unwrap();
println!("{}", password);
```

### Generate Public/Private Keys
```rust
use smartpasslib::{generate_private_key, generate_public_key};

let secret = "MyStrongSecretPhrase2026!";

let public_key = generate_public_key(secret).unwrap();
let private_key = generate_private_key(secret).unwrap();
```

### Verify Secret
```rust
use smartpasslib::{generate_public_key, verify_secret};

let secret = "MyStrongSecretPhrase2026!";
let stored_public_key = "...";

let is_valid = verify_secret(secret, &stored_public_key).unwrap();
```

### Generate Random
```rust
use smartpasslib::{generate_strong_password, generate_code};

let strong = generate_strong_password(20).unwrap();
let code = generate_code(8).unwrap();
```

### Metadata Manager
```rust
use smartpasslib::{SmartPasswordManager, SmartPassword};

let mut manager = SmartPasswordManager::new().unwrap();

let sp = SmartPassword::new(public_key, "GitHub".to_string(), 16).unwrap();
manager.add(sp).unwrap();

let retrieved = manager.get(&public_key).unwrap();
manager.delete(&public_key).unwrap();
```

---

## API Reference

### Constants

| Constant | Type | Description |
|----------|------|-------------|
| `VERSION` | &str | Library version (4.0.0) |
| `CHARS` | &str | Character set used for generation |

### Functions

| Function | Parameters | Returns | Description |
|----------|------------|---------|-------------|
| `generate_private_key(secret)` | secret: &str | Result\<String, Error\> | Private key (15-30 iterations) |
| `generate_public_key(secret)` | secret: &str | Result\<String, Error\> | Public key (45-60 iterations) |
| `verify_secret(secret, public_key)` | secret, public_key | Result\<bool, Error\> | Verify secret matches public key |
| `generate_smart_password_sync(secret, length)` | secret, length | Result\<String, Error\> | Deterministic password |
| `generate_strong_password(length)` | length: usize | Result\<String, Error\> | Cryptographically random |
| `generate_base_password(length)` | length: usize | Result\<String, Error\> | Simple random password |
| `generate_code(length)` | length: usize | Result\<String, Error\> | Short code (4-100 chars) |

### Classes

| Class | Description |
|-------|-------------|
| `SmartPassword` | Metadata container (public_key, description, length) |
| `SmartPasswordManager` | Persistent storage for password metadata (JSON file) |

---

## Examples

```bash
# Run demo with interactive menu
cargo run --example demo
```

Output:
```
========================================
  SmartPassLib v4.0.0 Demo
========================================

+--------------------------------------------------+
|              SMART PASS GENERATOR                |
+--------------------------------------------------+
|  1. Generate Smart Password                      |
|  2. Generate Random Password                     |
|  3. Generate 2FA Code                            |
|  4. Generate Key Pair                            |
|  5. Verify Secret                                |
|  0. Exit                                         |
+--------------------------------------------------+

Select option: 1

+--------------------------------------------------+
|           SMART PASSWORD GENERATOR               |
+--------------------------------------------------+

Enter secret phrase: MyStrongSecretPhrase2026!
Password length [16]: 16

  [OK] Password generated!
  Length: 16
  Password: ,9;?/SMtORgwSZ.5
  Time: 655us

  Public key (for storage):
  ac8d93d6b4b79b100e9e254c6dd1511cb22dfb5981c90b79409a22cc03460f07

Press Enter to continue...

+--------------------------------------------------+
|              SMART PASS GENERATOR                |
+--------------------------------------------------+
|  1. Generate Smart Password                      |
|  2. Generate Random Password                     |
|  3. Generate 2FA Code                            |
|  4. Generate Key Pair                            |
|  5. Verify Secret                                |
|  0. Exit                                         |
+--------------------------------------------------+

Select option: 0

  Goodbye!
```

```bash
# Run benchmarks
cargo bench

# Run tests
cargo test

# Build documentation
cargo doc --open
```

---

## Cross-Platform Implementations

The same deterministic algorithm is available in multiple languages.
SmartPassLib Rust produces **identical passwords** to:

| Language | Repository |
|----------|------------|
| Python | [smartpasslib](https://github.com/smartlegionlab/smartpasslib) |
| JavaScript | [smartpasslib-js](https://github.com/smartlegionlab/smartpasslib-js) |
| Kotlin | [smartpasslib-kotlin](https://github.com/smartlegionlab/smartpasslib-kotlin) |
| Go | [smartpasslib-go](https://github.com/smartlegionlab/smartpasslib-go) |
| C# | [smartpasslib-csharp](https://github.com/smartlegionlab/smartpasslib-csharp) |
| **Rust** | **smartpasslib** (this) |

---

## Ecosystem

**Core Libraries:**
- **[smartpasslib](https://github.com/smartlegionlab/smartpasslib)** - Python
- **[smartpasslib-js](https://github.com/smartlegionlab/smartpasslib-js)** - JavaScript
- **[smartpasslib-kotlin](https://github.com/smartlegionlab/smartpasslib-kotlin)** - Kotlin
- **[smartpasslib-go](https://github.com/smartlegionlab/smartpasslib-go)** - Go
- **[smartpasslib-csharp](https://github.com/smartlegionlab/smartpasslib-csharp)** - C#
- **smartpasslib** - Rust (this)

**CLI Applications:**
- **[CLI Smart Password Manager (Python)](https://github.com/smartlegionlab/clipassman)**
- **[CLI Smart Password Generator (Python)](https://github.com/smartlegionlab/clipassgen)**
- **[CLI Smart Password Manager (C#)](https://github.com/smartlegionlab/SmartPasswordManagerCsharpCli)**
- **[CLI Smart Password Generator (C#)](https://github.com/smartlegionlab/SmartPasswordGeneratorCsharpCli)**

**Desktop Applications:**
- **[Desktop Smart Password Manager (Python)](https://github.com/smartlegionlab/smart-password-manager-desktop)**
- **[Desktop Smart Password Manager (C#)](https://github.com/smartlegionlab/SmartPasswordManagerCsharpDesktop)**

**Other:**
- **[Smart Password Web Manager](https://github.com/smartlegionlab/smart-password-manager-web)**
- **[Smart Password Android Manager](https://github.com/smartlegionlab/smart-password-manager-android)**

---

## Development

```bash
git clone https://github.com/smartlegionlab/smartpasslib-rs
cd smartpasslib-rs

cargo build
cargo test
cargo run --example demo
cargo bench
cargo doc --open
```

### Commands

```bash
# Compilation check
cargo check

# Run tests
cargo test

# Run demo
cargo run --example demo

# Run benchmarks
cargo bench

# Documentation
cargo doc --open

# Format
cargo fmt

# Linter
cargo clippy
```

---

## License

[BSD 3-Clause License](LICENSE)

Copyright © 2026, [Alexander Suvorov](https://github.com/smartlegionlab)

---

## Author

**Alexander Suvorov**

- GitHub: [smartlegionlab](https://github.com/smartlegionlab)
- Website: [smartlegionlab.com](https://smartlegionlab.com)

---

## Support

- **Issues**: [GitHub Issues](https://github.com/smartlegionlab/smartpasslib-rs/issues)

