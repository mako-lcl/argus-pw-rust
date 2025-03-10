use anyhow::Result;
use anyhow::anyhow;
use clap::Parser;
use cli::{Cli, Commands};
use cli_clipboard::get_contents;
use cli_clipboard::set_contents;
use clipboard_anywhere::set_clipboard;
use config::Config;
use pw::generate_password;
use setup::setup_config;
use std::env;

mod cli;
mod config;
mod pw;
mod setup;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Gen {
            length,
            setup,
            display,
        } => {
            println!("Generating password of length {}", length);

            // -- load config
            let mut config = Config::default();
            if *setup {
                config = setup_config();
            }

            // -- gen pw
            let pw = match generate_password(config, *length) {
                Ok(pw) => pw,
                Err(e) => {
                    println!("Failed generating password: {:?}", e);
                    return;
                }
            };

            if *display {
                println!("{}", pw);
            }

            // Set the contents of the clipboard.
            if let Err(e) = copy_to_clipboard(&pw) {
                println!("{:?}", e);
                if !display {
                    println!("Password: {}", pw);
                }
            } else {
                println!("Password copied to clipboard");
            }
        }
    }
}

fn copy_to_clipboard(pw: &str) -> Result<()> {
    if is_wayland() {
        copy_to_clipboard_wayland(pw)
    } else {
        copy_to_clipboard_non_wayland(pw)
    }
}

fn copy_to_clipboard_wayland(pw: &str) -> Result<()> {
    if let Err(e) = set_contents(pw.to_owned()) {
        return Err(anyhow!("Failed to set clipboard contents: {:?}", e));
    }

    let _ = get_contents();

    Ok(())
}

fn copy_to_clipboard_non_wayland(pw: &str) -> Result<()> {
    if let Err(e) = set_clipboard(pw) {
        return Err(anyhow!("Failed to set clipboard contents: {:?}", e));
    }

    Ok(())
}

fn is_wayland() -> bool {
    // Check the session type
    if let Ok(session_type) = env::var("XDG_SESSION_TYPE") {
        if session_type.to_lowercase() == "wayland" {
            return true;
        }
    }
    // Fallback: Check if WAYLAND_DISPLAY is set
    env::var("WAYLAND_DISPLAY").is_ok()
}
