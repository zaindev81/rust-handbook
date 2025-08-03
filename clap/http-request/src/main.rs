use clap::Parser;

#[derive(Parser)]
#[command(name = "http-request")]
#[command(about = "A http-request tool")]
#[command(version = "1.0.0")]
#[command(author = "Your Name <your.email@example.com>")]struct Cli {
    /// Show statistics
    #[arg(long)]
    stats: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    Ok(())
}
