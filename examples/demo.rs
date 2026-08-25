//! Demo example for SmartPassLib
//!
//! Run: cargo run --example demo

use smartpasslib::*;
use std::io::{self, Write};
use std::time::Instant;

fn main() -> Result<()> {
    println!("========================================");
    println!("  SmartPassLib v{} Demo", VERSION);
    println!("========================================\n");

    loop {
        println!("\n+--------------------------------------------------+");
        println!("|              SMART PASS GENERATOR                |");
        println!("+--------------------------------------------------+");
        println!("|  1. Generate Smart Password                      |");
        println!("|  2. Generate Random Password                     |");
        println!("|  3. Generate 2FA Code                            |");
        println!("|  4. Generate Key Pair                            |");
        println!("|  5. Verify Secret                                |");
        println!("|  0. Exit                                         |");
        println!("+--------------------------------------------------+");
        print!("\nSelect option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => generate_smart_password_menu()?,
            "2" => generate_random_password_menu()?,
            "3" => generate_code_menu()?,
            "4" => generate_key_pair_menu()?,
            "5" => verify_secret_menu()?,
            "0" => {
                println!("\n  Goodbye!");
                break;
            }
            _ => println!("\n  Invalid option. Please try again."),
        }
    }

    Ok(())
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn get_input_with_default(prompt: &str, default: &str) -> String {
    let input = get_input(&format!("{} [{}]: ", prompt, default));
    if input.is_empty() {
        default.to_string()
    } else {
        input
    }
}

fn generate_smart_password_menu() -> Result<()> {
    println!("\n+--------------------------------------------------+");
    println!("|           SMART PASSWORD GENERATOR               |");
    println!("+--------------------------------------------------+\n");

    let secret = get_input("Enter secret phrase: ");
    if secret.len() < 12 {
        println!("  [ERROR] Secret must be at least 12 characters!");
        return Ok(());
    }

    let length_str = get_input_with_default("Password length", "16");
    let length = length_str.parse().unwrap_or(16);

    let start = Instant::now();
    match generate_smart_password_sync(&secret, length) {
        Ok(password) => {
            let elapsed = start.elapsed();
            println!("\n  [OK] Password generated!");
            println!("  Length: {}", length);
            println!("  Password: {}", password);
            println!("  Time: {}us", elapsed.as_micros());
            println!("\n  Public key (for storage):");
            match generate_public_key(&secret) {
                Ok(pk) => println!("  {}", pk),
                Err(e) => println!("  [ERROR] {}", e),
            }
        }
        Err(e) => println!("  [ERROR] {}", e),
    }

    pause();
    Ok(())
}

fn generate_random_password_menu() -> Result<()> {
    println!("\n+--------------------------------------------------+");
    println!("|           RANDOM PASSWORD GENERATOR              |");
    println!("+--------------------------------------------------+\n");

    let length_str = get_input_with_default("Password length", "16");
    let length = length_str.parse().unwrap_or(16);

    match generate_strong_password(length) {
        Ok(password) => {
            println!("\n  [OK] Random password generated!");
            println!("  Length: {}", length);
            println!("  Password: {}", password);
        }
        Err(e) => println!("  [ERROR] {}", e),
    }

    pause();
    Ok(())
}

fn generate_code_menu() -> Result<()> {
    println!("\n+--------------------------------------------------+");
    println!("|              2FA CODE GENERATOR                  |");
    println!("+--------------------------------------------------+\n");

    let length_str = get_input_with_default("Code length", "6");
    let length = length_str.parse().unwrap_or(6);

    match generate_code(length) {
        Ok(code) => {
            println!("\n  [OK] 2FA Code generated!");
            println!("  Length: {}", length);
            println!("  Code: {}", code);
        }
        Err(e) => println!("  [ERROR] {}", e),
    }

    pause();
    Ok(())
}

fn generate_key_pair_menu() -> Result<()> {
    println!("\n+--------------------------------------------------+");
    println!("|              KEY PAIR GENERATOR                  |");
    println!("+--------------------------------------------------+\n");

    let secret = get_input("Enter secret phrase: ");
    if secret.len() < 12 {
        println!("  [ERROR] Secret must be at least 12 characters!");
        return Ok(());
    }

    match generate_private_key(&secret) {
        Ok(private) => match generate_public_key(&secret) {
            Ok(public) => {
                println!("\n  [OK] Key pair generated!");
                println!("  Private key: {}", private);
                println!("  Public key:  {}", public);
                println!("\n  Save public key for verification.");
            }
            Err(e) => println!("  [ERROR] {}", e),
        },
        Err(e) => println!("  [ERROR] {}", e),
    }

    pause();
    Ok(())
}

fn verify_secret_menu() -> Result<()> {
    println!("\n+--------------------------------------------------+");
    println!("|             SECRET VERIFICATION                  |");
    println!("+--------------------------------------------------+\n");

    let secret = get_input("Enter secret phrase: ");
    let public_key = get_input("Enter public key: ");

    match verify_secret(&secret, &public_key) {
        Ok(true) => println!("\n  [OK] Secret is VALID!"),
        Ok(false) => println!("\n  [FAIL] Secret is INVALID!"),
        Err(e) => println!("  [ERROR] {}", e),
    }

    pause();
    Ok(())
}

fn pause() {
    print!("\nPress Enter to continue...");
    io::stdout().flush().unwrap();
    let mut _input = String::new();
    io::stdin().read_line(&mut _input).unwrap();
}
