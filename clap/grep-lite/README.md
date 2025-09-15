# grep-lite

```sh
# Basic search
cargo run -- "Error" log1.txt log2.txt

# With line numbers and case sensitivity
cargo run -- "Error" log1.txt --line-numbers --case-sensitive
cargo run -- "Error" log1.txt log2.txt --line-numbers --case-sensitive

# examples
cargo run -- "Error" log1.txt --line-numbers
cargo run -- "Error" log1.txt --line-numbers --case-sensitive
cargo run -- "Error" log1.txt --line-numbers --case-sensitive --count
cargo run -- "Error" log1.txt --line-numbers --invert


# Count matches only
cargo run -- "warning" app.log --count

# Show non-matching lines
cargo run -- "debug" app.log --invert

# Combine flags
cargo run -- "exception" log1.txt log2.txt --line-numbers --case-sensitive --count
```

Test

```sh
cargo test
```
