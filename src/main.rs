use clap::Parser;
use cli::{Cli, Commands};
use cli_clipboard::set_contents;
use config::Config;
use pw::generate_password;
use setup::setup_config;

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
            let pw = generate_password(config, *length);

            if *display {
                println!("{}", pw);
            }

            // Set the contents of the clipboard.
            set_contents(pw.to_owned()).expect("Failed copying to clipboard!");
            assert_eq!(cli_clipboard::get_contents().unwrap(), pw);

            println!("Password copied to clipboard");
        }
    }
}
