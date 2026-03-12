use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};


#[derive(Parser)]
#[command(name = "mygit", about = "A minimal git-like VCS", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialise a new repository in the current directory
    Init,
    /// Stage a file for the next commit
    Add { file: String },
    /// Commit all staged files with a message
    Commit {
        #[arg(short, long)]
        message: String,
    },
    /// Show the commit history
    Log,
    /// Show currently staged files
    Status,
}


#[derive(Serialize, Deserialize)]
struct Commit {
    id: String,
    message: String,
    timestamp: u64,
    files: Vec<String>,
}


fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Init => init_repo(),
        Commands::Add { file } => add_file(&file),
        Commands::Commit { message } => commit(&message),
        Commands::Log => log_commits(),
        Commands::Status => status(),
    };

    if let Err(e) = result {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

const MYGIT_DIR: &str = ".mygit";
const COMMITS_DIR: &str = ".mygit/commits";
const INDEX_FILE: &str = ".mygit/index";

/// Return an error if the repository has not been initialised yet.
fn require_repo() -> Result<(), String> {
    if !Path::new(MYGIT_DIR).exists() {
        return Err("not a mygit repository (run `mygit init` first)".into());
    }
    Ok(())
}

fn generate_commit_id(message: &str, timestamp: u64) -> String {
    let mut hasher = Sha256::new();
    hasher.update(message.as_bytes());
    hasher.update(timestamp.to_string().as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)[..8].to_string()
}

fn format_timestamp(secs: u64) -> String {

    let s = secs % 60;
    let m = (secs / 60) % 60;
    let h = (secs / 3600) % 24;
    let days_since_epoch = secs / 86400;
    // Rough Gregorian calculation (good enough for display).
    let year = 1970 + days_since_epoch / 365;
    let day_of_year = days_since_epoch % 365 + 1;
    format!(
        "year ~{year}, day {day_of_year}  {:02}:{:02}:{:02} UTC",
        h, m, s
    )
}


fn init_repo() -> Result<(), String> {
    if Path::new(MYGIT_DIR).exists() {
        return Err("repository already initialised".into());
    }
    fs::create_dir(MYGIT_DIR).map_err(|e| format!("cannot create {MYGIT_DIR}: {e}"))?;
    fs::create_dir(COMMITS_DIR).map_err(|e| format!("cannot create {COMMITS_DIR}: {e}"))?;
    fs::write(INDEX_FILE, "").map_err(|e| format!("cannot create index: {e}"))?;
    println!("Initialised empty repository in {MYGIT_DIR}/");
    Ok(())
}

fn add_file(file: &str) -> Result<(), String> {
    require_repo()?;

    // Make sure the file actually exists before staging it.
    if !Path::new(file).exists() {
        return Err(format!("'{file}' did not match any files"));
    }

    // Avoid staging the same file twice.
    let index_contents =
        fs::read_to_string(INDEX_FILE).map_err(|e| format!("cannot read index: {e}"))?;

    if index_contents.lines().any(|l| l == file) {
        println!("'{file}' is already staged");
        return Ok(());
    }

    let mut index = OpenOptions::new()
        .append(true)
        .open(INDEX_FILE)
        .map_err(|e| format!("cannot open index: {e}"))?;

    writeln!(index, "{file}").map_err(|e| format!("cannot write index: {e}"))?;
    println!("Staged '{file}'");
    Ok(())
}

fn commit(message: &str) -> Result<(), String> {
    require_repo()?;

    let staged =
        fs::read_to_string(INDEX_FILE).map_err(|e| format!("cannot read index: {e}"))?;

    let files: Vec<String> = staged
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(String::from)
        .collect();

    if files.is_empty() {
        println!("Nothing to commit (staging area is empty)");
        return Ok(());
    }

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| format!("system clock error: {e}"))?
        .as_secs();

    let commit_id = generate_commit_id(message, timestamp);

    let commit = Commit {
        id: commit_id.clone(),
        message: message.to_string(),
        timestamp,
        files: files.clone(),
    };

    let json =
        serde_json::to_string_pretty(&commit).map_err(|e| format!("serialisation error: {e}"))?;

    let commit_path = format!("{COMMITS_DIR}/{commit_id}.json");
    fs::write(&commit_path, json).map_err(|e| format!("cannot write commit file: {e}"))?;

    // Clear the staging area.
    fs::write(INDEX_FILE, "").map_err(|e| format!("cannot clear index: {e}"))?;

    println!("Committed {} file(s) — id {commit_id}", files.len());
    Ok(())
}

fn log_commits() -> Result<(), String> {
    require_repo()?;

    let mut commits: Vec<Commit> = Vec::new();

    for entry in
        fs::read_dir(COMMITS_DIR).map_err(|e| format!("cannot read commits dir: {e}"))?
    {
        let path = entry
            .map_err(|e| format!("cannot read directory entry: {e}"))?
            .path();

        if path.extension().and_then(|s| s.to_str()) != Some("json") {
            continue;
        }

        let data =
            fs::read_to_string(&path).map_err(|e| format!("cannot read {path:?}: {e}"))?;

        let commit: Commit =
            serde_json::from_str(&data).map_err(|e| format!("malformed commit {path:?}: {e}"))?;

        commits.push(commit);
    }

    if commits.is_empty() {
        println!("No commits yet.");
        return Ok(());
    }

    // Show newest first.
    commits.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

    for commit in &commits {
        println!("commit {}", commit.id);
        println!("date:    {}", format_timestamp(commit.timestamp));
        println!("message: {}", commit.message);
        println!("files:   {}", commit.files.join(", "));
        println!();
    }

    Ok(())
}

fn status() -> Result<(), String> {
    require_repo()?;

    let staged =
        fs::read_to_string(INDEX_FILE).map_err(|e| format!("cannot read index: {e}"))?;

    let files: Vec<&str> = staged
        .lines()
        .filter(|l| !l.trim().is_empty())
        .collect();

    if files.is_empty() {
        println!("Nothing staged for commit.");
    } else {
        println!("Staged files ({}):", files.len());
        for f in &files {
            println!("  + {f}");
        }
    }

    Ok(())
}