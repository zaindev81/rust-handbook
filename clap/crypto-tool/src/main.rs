use clap::{Parser, Subcommand};
use std::fs;
use std::io::{self, Write};
use std::path::Path;

use base64::{Engine as _, engine::general_purpose};

#[derive(Parser, Debug)]
#[command(name = "crypto-tool")]
#[command(about = "A cryptographic tool for file encryption/decryption")]
#[command(version = "1.0")]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
     #[command(about = "Encrypt a file")]
    Encrypt {
        /// Input file path
        #[arg(short = 'i', long = "input", value_name = "FILE")]
        input: String,
    },

    #[command(about = "Decrypt a file")]
    Decrypt {
        /// Input file path
        #[arg(short = 'i', long = "input", value_name = "FILE")]
        input: String,
    }
}

#[derive(Debug)]
enum CryptoError {
    IoError(io::Error),
    CryptoError(String),
    Base64Error(base64::DecodeError),
    InvalidKeyLength,
    InvalidDataLength,
}

impl From<io::Error> for CryptoError {
    fn from(err: io::Error) -> Self {
        CryptoError::IoError(err)
    }
}

impl From<base64::DecodeError> for CryptoError {
    fn from(err: base64::DecodeError) -> Self {
        CryptoError::Base64Error(err)
    }
}

impl std::fmt::Display for CryptoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CryptoError::IoError(e) => write!(f, "IO Error: {}", e),
            CryptoError::CryptoError(e) => write!(f, "Crypto Error: {}", e),
            CryptoError::Base64Error(e) => write!(f, "Base64 Error: {}", e),
            CryptoError::InvalidKeyLength => write!(f, "Invalid key length"),
            CryptoError::InvalidDataLength => write!(f, "Invalid data length"),
        }
    }
}

type Result<T> = std::result::Result<T, CryptoError>;
impl std::error::Error for CryptoError {}

fn main() {
    let args = Args::parse();

   let result = match args.command {
        Commands::Encrypt { input } => handle_encrypt(&input),
        Commands::Decrypt { input } => handle_decrypt(&input),
    };

    // println!("{:?}", args.command)

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn handle_encrypt(input: &str) -> Result<()> {
    print!("handle_encrypt");
    
    Ok(())
}

fn handle_decrypt(input: &str) -> Result<()> {
    print!("handle_decrypt");

    Ok(())
}