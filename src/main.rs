mod cli;
mod cmd;
pub mod draft;
pub mod loader;

use clap::{Parser, Subcommand};
use cli::Command;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    /// Turn debugging information on
    #[arg(short, long)]
    debug: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Generate {
        #[arg(short, long)]
        file: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.debug {
        false => println!("Debug mode is off"),
        true => println!("Debug mode is on"),
    }

    match &cli.command {
        Some(Commands::Generate { file }) => {
            cmd::generate::Command {
                file: PathBuf::from(file),
            }
            .run();
        }
        None => {}
    }

    println!("DONE!")
}
