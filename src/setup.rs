use crate::config::Config;
use std::error::Error;
use std::io::{Write, stdin, stdout};

/// Prompts the user to configure the password generation settings.
pub fn setup_config() -> Config {
    // Start with default configuration.
    let mut config = Config::default().clone();

    // Ask user for each option.
    config.use_uppercase = prompt_bool("Use uppercase letters? (y/n): ").expect("Unexpected error");
    config.use_lowercase = prompt_bool("Use lowercase letters? (y/n): ").expect("Unexpected error");
    config.use_digits = prompt_bool("Use digits? (y/n): ").expect("Unexpected error");
    config.use_special = prompt_bool("Use special characters? (y/n): ").expect("Unexpected error");

    // If enabled, ask for custom character sets. Pressing Enter keeps the default.
    if config.use_uppercase {
        config.uppercase = prompt_string(
            &format!(
                "Enter uppercase characters (empty for default: {}): ",
                config.uppercase
            ),
            &config.uppercase,
        )
        .expect("Unexpected error");
    }
    if config.use_lowercase {
        config.lowercase = prompt_string(
            &format!(
                "Enter lowercase characters (empty for default: {}): ",
                config.lowercase
            ),
            &config.lowercase,
        )
        .expect("Unexpected error");
    }
    if config.use_digits {
        config.digits = prompt_string(
            &format!("Enter digits (empty for default: {}): ", config.digits),
            &config.digits,
        )
        .expect("Unexpected error");
    }
    if config.use_special {
        config.special = prompt_string(
            &format!(
                "Enter special characters (empty for default: {}): ",
                config.special
            ),
            &config.special,
        )
        .expect("Unexpected error");
    }

    config
}

/// Prompts the user with a yes/no question and returns the answer as a boolean.
fn prompt_bool(prompt: &str) -> Result<bool, Box<dyn Error>> {
    let mut input = String::new();
    print!("{}", prompt);
    stdout().flush()?;
    stdin().read_line(&mut input)?;
    let input = input.trim().to_lowercase();
    match input.as_str() {
        "y" | "yes" => Ok(true),
        "n" | "no" => Ok(false),
        _ => {
            println!("Invalid input. Please enter 'y' or 'n'.");
            prompt_bool(prompt)
        }
    }
}

/// Prompts the user for a string input, returning the default if the input is empty.
fn prompt_string(prompt: &str, default: &str) -> Result<String, Box<dyn Error>> {
    let mut input = String::new();
    print!("{}", prompt);
    stdout().flush()?;
    stdin().read_line(&mut input)?;
    let input = input.trim();
    if input.is_empty() {
        Ok(default.to_string())
    } else {
        Ok(input.to_string())
    }
}
