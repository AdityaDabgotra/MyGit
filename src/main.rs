use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    Add { file: String },
    Commit { message: String },
    Log,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => init_repo(),
        Commands::Add { file } => println!("Adding file {}", file),
        Commands::Commit { message } => println!("Commit message: {}", message),
        Commands::Log => println!("Showing commit log"),
    }
}

use std::fs;

fn init_repo() {
    fs::create_dir(".mygit").unwrap();
    fs::create_dir(".mygit/commits").unwrap();
    fs::write(".mygit/index", "").unwrap();

    println!("Initialized empty repository");
}