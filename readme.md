<div align="center">

```
███╗   ███╗██╗   ██╗ ██████╗ ██╗████████╗
████╗ ████║╚██╗ ██╔╝██╔════╝ ██║╚══██╔══╝
██╔████╔██║ ╚████╔╝ ██║  ███╗██║   ██║   
██║╚██╔╝██║  ╚██╔╝  ██║   ██║██║   ██║   
██║ ╚═╝ ██║   ██║   ╚██████╔╝██║   ██║   
╚═╝     ╚═╝   ╚═╝    ╚═════╝ ╚═╝   ╚═╝   
```

### 🦀 A minimal, blazing-fast Git-like version control system — built in Rust

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue?style=for-the-badge)](LICENSE)
[![Build](https://img.shields.io/badge/Build-Passing-brightgreen?style=for-the-badge&logo=github-actions&logoColor=white)](https://github.com/AdityaDabgotra/MyGit)
[![GitHub](https://img.shields.io/badge/GitHub-AdityaDabgotra-black?style=for-the-badge&logo=github)](https://github.com/AdityaDabgotra/MyGit)

<br/>

> *"Because sometimes you want to understand how Git actually works under the hood."*

</div>

---

## 📖 Table of Contents

- [✨ What is MyGit?](#-what-is-mygit)
- [🚀 Features](#-features)
- [📦 Installation](#-installation)
- [🛠️ Usage](#️-usage)
- [💡 Commands Reference](#-commands-reference)
- [📁 Project Structure](#-project-structure)
- [🔧 How It Works](#-how-it-works)
- [🏗️ Tech Stack](#️-tech-stack)
- [🤝 Contributing](#-contributing)
- [📄 License](#-license)

---

## ✨ What is MyGit?

**MyGit** is a lightweight, terminal-based version control system inspired by Git, written entirely in **Rust**. It lets you track file changes, stage files, commit snapshots, and view your project's history — all from the command line.

This project is a great way to:

- 🧠 **Learn** how version control systems work internally
- 🦀 **Explore** Rust's file I/O, error handling, and CLI tooling
- 🔨 **Build** on top of a clean, minimal VCS foundation

---

## 🚀 Features

| Feature | Description |
|---|---|
| `init` | Initialise a new repository in any directory |
| `add` | Stage files for the next commit (with duplicate and existence checks) |
| `commit` | Snapshot staged files with a message and SHA-256 commit ID |
| `log` | View full commit history sorted newest-first |
| `status` | See what files are currently staged |
| 🔒 **Safe errors** | No panics — every command returns clear, friendly error messages |
| 🆔 **Content-addressed IDs** | Commit IDs generated via SHA-256 hash of message + timestamp |
| 💾 **JSON storage** | Commits stored as human-readable `.json` files |

---

## 📦 Installation

### Prerequisites

Make sure you have **Rust and Cargo** installed. If not:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

Verify your installation:

```bash
rustc --version
cargo --version
```

### Clone & Build

```bash
# 1. Clone the repository
git clone https://github.com/AdityaDabgotra/MyGit.git
cd MyGit

# 2. Build in release mode (optimised binary)
cargo build --release

# 3. (Optional) Install globally so you can run `mygit` anywhere
cargo install --path .
```

The compiled binary will be at `./target/release/mygit`.

---

## 🛠️ Usage

### Quick Start — from scratch

```bash
# Step 1: Create a working directory and enter it
mkdir my-project && cd my-project

# Step 2: Initialise a MyGit repository
mygit init
# → Initialised empty repository in .mygit/

# Step 3: Create a file to track
echo "Hello, MyGit!" > hello.txt

# Step 4: Stage the file
mygit add hello.txt
# → Staged 'hello.txt'

# Step 5: Check what's staged
mygit status
# → Staged files (1):
#     + hello.txt

# Step 6: Commit your changes
mygit commit -m "Add hello.txt"
# → Committed 1 file(s) — id a3f92c1b

# Step 7: View your commit history
mygit log
# → commit a3f92c1b
#    date:    year ~2025, day 180  14:32:10 UTC
#    message: Add hello.txt
#    files:   hello.txt
```

### Running without installing globally

If you haven't run `cargo install --path .`, prefix every command with the binary path:

```bash
./target/release/mygit init
./target/release/mygit add hello.txt
./target/release/mygit commit -m "first commit"
```

Or use `cargo run` during development:

```bash
cargo run -- init
cargo run -- add hello.txt
cargo run -- commit -m "first commit"
cargo run -- log
cargo run -- status
```

---

## 💡 Commands Reference

### `mygit init`

Initialises a new repository by creating the `.mygit/` directory structure.

```bash
mygit init
```

```
.mygit/
├── commits/    ← commit JSON files are stored here
└── index       ← staging area (list of staged files)
```

> ⚠️ Throws an error if a repository already exists in the current directory.

---

### `mygit add <file>`

Stages a file for the next commit.

```bash
mygit add src/main.rs
mygit add README.md
```

**Guards:**
- ❌ File does not exist on disk → error
- ❌ File is already staged → skips with a message
- ✅ Otherwise → appends to `.mygit/index`

---

### `mygit commit -m "<message>"`

Creates a commit snapshot of all staged files.

```bash
mygit commit -m "Fix login bug"
mygit commit -m "Add dark mode support"
```

**What happens internally:**
1. Reads all staged files from `.mygit/index`
2. Generates a unique 8-character commit ID using `SHA-256(message + timestamp)`
3. Saves a `.json` file to `.mygit/commits/<id>.json`
4. Clears the staging area

> ⚠️ Prints `"Nothing to commit"` if the staging area is empty.

---

### `mygit log`

Displays full commit history, sorted **newest first**.

```bash
mygit log
```

**Sample output:**

```
commit f7a3d91e
date:    year ~2025, day 201  09:15:42 UTC
message: Refactor CLI commands
files:   src/main.rs, src/lib.rs

commit a3f92c1b
date:    year ~2025, day 180  14:32:10 UTC
message: Add hello.txt
files:   hello.txt
```

---

### `mygit status`

Shows all files currently staged and waiting to be committed.

```bash
mygit status
```

**Sample output:**

```
Staged files (2):
  + src/main.rs
  + Cargo.toml
```

> Prints `"Nothing staged for commit."` if the staging area is empty.

---

## 📁 Project Structure

```
MyGit/
├── src/
│   └── main.rs         ← All source code (CLI, commands, data model)
├── Cargo.toml          ← Dependencies and project metadata
├── Cargo.lock          ← Locked dependency versions
└── README.md           ← You are here
```

**Generated at runtime (inside any repo you initialise):**

```
your-project/
└── .mygit/
    ├── index                  ← Staging area
    └── commits/
        ├── a3f92c1b.json      ← Each commit as a JSON file
        └── f7a3d91e.json
```

**Commit JSON format:**

```json
{
  "id": "a3f92c1b",
  "message": "Add hello.txt",
  "timestamp": 1718900130,
  "files": [
    "hello.txt"
  ]
}
```

---

## 🔧 How It Works

```
┌─────────────────────────────────────────────────────────┐
│                    Working Directory                    │
│          (your files: main.rs, README.md, ...)          │
└────────────────────────┬────────────────────────────────┘
                         │  mygit add <file>
                         ▼
┌─────────────────────────────────────────────────────────┐
│                    Staging Area                         │
│               (.mygit/index — plain text)               │
└────────────────────────┬────────────────────────────────┘
                         │  mygit commit -m "..."
                         ▼
┌─────────────────────────────────────────────────────────┐
│                   Commit Store                          │
│         (.mygit/commits/<sha256-id>.json)               │
│   { id, message, timestamp, files[] }                   │
└─────────────────────────────────────────────────────────┘
                         │  mygit log
                         ▼
                  Terminal output
             (sorted newest → oldest)
```

1. **`init`** — Creates the `.mygit/` directory tree
2. **`add`** — Appends file paths to `.mygit/index` (the staging area)
3. **`commit`** — Reads the index, generates a SHA-256 ID, writes a JSON commit object, then clears the index
4. **`log`** — Reads all JSON files from `.mygit/commits/`, deserialises them, sorts by timestamp, and prints
5. **`status`** — Reads `.mygit/index` and lists the staged files

---

## 🏗️ Tech Stack

| Crate | Purpose |
|---|---|
| [`clap`](https://crates.io/crates/clap) `v4` | CLI argument parsing with derive macros |
| [`serde`](https://crates.io/crates/serde) + [`serde_json`](https://crates.io/crates/serde_json) | Commit serialisation / deserialisation |
| [`sha2`](https://crates.io/crates/sha2) | SHA-256 hashing for commit IDs |
| `std::fs` | File and directory I/O |
| `std::time` | Unix timestamp generation |

---

## 🤝 Contributing

Contributions are welcome! Here are some ideas for extending MyGit:

- 🌿 **Branching** — `mygit branch` and `mygit checkout`
- 🔀 **Diffing** — `mygit diff` to show what changed between commits
- 🏷️ **Tags** — `mygit tag <name>` to label specific commits
- ↩️ **Revert** — restore files to a previous commit
- 🌐 **Remote support** — push/pull over HTTP

To contribute:

```bash
# Fork the repo on GitHub, then:
git clone https://https://github.com/AdityaDabgotra/MyGit
cd MyGit
git checkout -b feature/my-feature
# make your changes
git commit -m "Add my feature"
git push origin feature/my-feature
# Open a Pull Request on GitHub
```

---

## 📄 License

This project is licensed under the **MIT License** — feel free to use, modify, and distribute it.

---

<div align="center">

Made with ❤️ and 🦀 by [Aditya Dabgotra](https://github.com/AdityaDabgotra)

⭐ **Star this repo if you found it useful!** ⭐

</div>