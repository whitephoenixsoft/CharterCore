use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "charter")]
#[command(about = "A CLI to version control your decisions.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Save the current engine state to a file NOT REAL
    Save {
        /// The path to the file where state will be saved
        path: PathBuf,
    },
    /// Load a state from a file (example of another command) NOT REAL
    Load {
        path: PathBuf,
    },
}

fn main() {
    println!("NOT OFFICIAL CLI; PLACEHOLDER CODE");
    println!("charter-cli 0.1.0; {}", charter_core::engine_version());
    
    let cli = Cli::parse();

    match &cli.command {
        Commands::Save { path } => {
            println!("Saving engine state to: {:?}", path);
            // logic to call your object store goes here
        }
        Commands::Load { path } => {
            println!("Loading state from: {:?}", path);
        }
    }
}
