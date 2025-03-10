use clap::{Parser, Subcommand};

/// A simple CLI tool
#[derive(Parser)]
#[command(name = "argus")]
#[command(author = "LCL")]
#[command(version = "1.0")]
#[command(about = "Generates passwords", long_about = None)]
pub struct Cli {
    /// The subcommand to run
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    // -- Generates password of length.
    Gen {
        length: u32,

        #[arg(
            short,
            long,
            help = "Additionally sets up character sets for generation"
        )]
        setup: bool,

        #[arg(short, long, help = "Displays the generated password in console")]
        display: bool,
    },
}
