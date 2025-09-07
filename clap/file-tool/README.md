# file-tool

A simple command-line file manipulation tool written in Rust.
Supports **copy**, **move**, and **delete** operations with optional flags for force and verbose output.

## Features
- **Copy files** with optional directory creation.
- **Move files** safely with overwrite protection.
- **Delete files or directories** with optional confirmation prompt.
- Verbose output for detailed logs.
- Force flag to overwrite or skip confirmations.

## Usage

```sh
# Copy a file
cargo run -- copy source.txt destination.txt

# Force overwrite with verbose output
cargo run -- copy source.txt destination.txt --force --verbose
cargo run -- copy source.txt dist/destination.txt --force --verbose

# Move a file
cargo run -- move old_name.txt unwanted.txt

# Delete a file (with confirmation)
cargo run -- delete unwanted.txt --verbose

# Force delete a directory
cargo run -- delete old_directory --force

# Verbose operation
cargo run -- copy data.txt backup/data_backup.txt --verbose
```