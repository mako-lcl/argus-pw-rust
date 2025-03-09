use clap::Parser;
use cli::{Cli, Commands};
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
        Commands::Gen { length, setup } => {
            println!("Generating password of length {}", length);

            // -- load config
            let mut config = Config::default();
            if *setup {
                config = setup_config();
            }

            // -- gen pw
            let pw = generate_password(config, *length);

            println!("{}", pw);
        }
    }
}
