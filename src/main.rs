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
        Commands::Add { file } => add_file(&file),
        Commands::Commit { message } => commit(&message),
        Commands::Log => log_commits(),
    }
}

use std::fs;

fn init_repo() {
    fs::create_dir(".mygit").unwrap();
    fs::create_dir(".mygit/commits").unwrap();
    fs::write(".mygit/index", "").unwrap();

    println!("Initialized empty repository");
}

use std::fs::OpenOptions;
use std::io::Write;

fn add_file(file: &str) {
    let mut index = OpenOptions::new()
        .append(true)
        .open(".mygit/index")
        .unwrap();

    writeln!(index, "{}", file).unwrap();

    println!("Added {}", file);
}

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Commit {
    id: String,
    message: String,
    timestamp: u64,
    files: Vec<String>,
}

use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};

fn generate_commit_id(message: &str, timestamp: u64) -> String {
    let mut hasher = Sha256::new();

    hasher.update(message);
    hasher.update(timestamp.to_string());

    let result = hasher.finalize();

    format!("{:x}", result)[0..8].to_string()
}

fn commit(message: &str) {
    // read staged files
    let index_path = ".mygit/index";

    let staged = fs::read_to_string(index_path)
        .expect("Failed to read staging area");

    let files: Vec<String> = staged
        .lines()
        .map(|s| s.to_string())
        .collect();

    if files.is_empty() {
        println!("Nothing to commit");
        return;
    }

    // timestamp
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // commit id
    let commit_id = generate_commit_id(message, timestamp);

    // create commit
    let commit = Commit {
        id: commit_id.clone(),
        message: message.to_string(),
        timestamp,
        files,
    };

    // serialize commit
    let json = serde_json::to_string_pretty(&commit)
        .expect("Failed to serialize commit");

    // save commit file
    let commit_path = format!(".mygit/commits/{}.json", commit_id);

    fs::write(commit_path, json)
        .expect("Failed to write commit");

    // clear staging area
    fs::write(index_path, "")
        .expect("Failed to clear staging area");

    println!("Committed successfully with id {}", commit_id);
}

fn log_commits() {
    for entry in fs::read_dir(".mygit/commits").unwrap() {
        let path = entry.unwrap().path();
        let data = fs::read_to_string(path).unwrap();

        let commit: Commit = serde_json::from_str(&data).unwrap();

        println!("commit {}", commit.id);
        println!("message: {}", commit.message);
        println!();
    }
}