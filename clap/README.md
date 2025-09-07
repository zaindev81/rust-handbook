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