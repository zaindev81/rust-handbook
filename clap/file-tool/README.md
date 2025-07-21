# file-tool


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