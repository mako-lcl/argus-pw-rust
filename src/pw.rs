use crate::config::Config;
use anyhow::Result;
use anyhow::anyhow;
use rand::Rng;

pub fn generate_password(config: Config, length: u32) -> Result<String> {
    // -- load usable characters
    let mut characters = String::new();

    if config.use_digits {
        characters.push_str(&config.digits);
    }

    if config.use_lowercase {
        characters.push_str(&config.lowercase);
    }

    if config.use_uppercase {
        characters.push_str(&config.uppercase);
    }

    if config.use_special {
        characters.push_str(&config.special);
    }

    if characters.is_empty() {
        return Err(anyhow!("No characters for password generation selected!"));
    }

    // -- loop and create pw
    let mut rng = rand::rng();
    let mut pw = String::new();
    let chars: Vec<char> = characters.chars().collect();

    for _ in 0..length {
        // -- get random subcharacter
        let random_char = chars[rng.random_range(0..chars.len())];

        pw.push(random_char);
    }

    Ok(pw)
}
