# clap

A full featured, fast Command Line Argument Parser for Rust

- https://github.com/clap-rs/clap
- https://github.com/clap-rs/clap/tree/master/examples
- https://docs.rs/clap/latest/clap/

## Topics

- 1. File Operations Tool `file-tool`
- 2. Text Search Tool `grep-lite`
- 3. Encryption Tool `crypto-tool`
- 4. Log Analysis Tool `log-analyzer`
- 5. HTTP Request Tool `http-client`
- 6. Image Resize Tool `img-resize`
- 7. Database Migration Tool `db-migrate`
- 8. Code Quality Checker `code-lint`
- 9. Backup Tool `backup-tool`
- 10. System Monitor Tool `sys-monitor`

## Other Topics

1. `file-sync` – Synchronize files between directories or remote servers
2. `csv-analyzer` – Analyze CSV files
3. `json-formatter` – Pretty-print or minify JSON files
4. `test-runner` – Run unit tests in multiple languages
5. `dep-check` – Check for outdated dependencies in projects
6. `docker-helper` – Manage Docker images and containers
7. `net-scan` – Scan local or remote networks for active hosts
8. `sys-info` – Show system information (CPU, memory, disk usage)
9. `port-check` – Check if a specific port is open on a host
10. `hash-tool` – Generate or verify file hashes (MD5, SHA256)
11. `jwt-inspector` – Decode and inspect JWT tokens
12. `password-gen` – Generate random secure passwords
13. `log-summarizer` – Summarize large log files by error frequency
14. `task-scheduler` – Run tasks on a schedule (cron-like tool)
15. `data-cleaner` – Clean and normalize datasets for ML pipelines
16. `note-cli` – Create and manage text notes in terminal
17. `time-tracker` – Track time spent on tasks and projects
18. `todo-cli` – Simple to-do list manager

## Example

```sh
cargo add clap --features derive
```

`main.rs`

```rust
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    for i in 0..args.count {
        println!("Hello {}! ({})", args.name, i + 1);
    }
}
```

`command`

```sh
cargo run -- --name world --count 10
```