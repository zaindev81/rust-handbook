use clap::{Parser, Subcommand};
use base64::{Engine as _, engine::general_purpose};

use crypto_tool::file::{read_input_file, write_output};
use crypto_tool::types::{CryptoMethod, Result};
use crypto_tool::encrypt::{encrypt_data, decrypt_data};

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

        /// Output file path (optional, defaults to stdout)
        #[arg(short = 'o', long = "output", value_name = "FILE")]
        output: Option<String>,

        /// Encryption method
        #[arg(long = "method", value_enum, default_value = "aes")]
        method: CryptoMethod,

        /// Password for encryption
        #[arg(long = "password")]
        password: String,

        /// Enable Base64 encoding
        #[arg(long = "base64")]
        base64: bool,
    },

    #[command(about = "Decrypt a file")]
    Decrypt {
        /// Input file path
        #[arg(short = 'i', long = "input", value_name = "FILE")]
        input: String,

        /// Output file path (optional, defaults to stdout)
        #[arg(short = 'o', long = "output", value_name = "FILE")]
        output: Option<String>,

        /// Encryption method
        #[arg(long = "method", value_enum, default_value = "aes")]
        method: CryptoMethod,

        /// Password for decryption
        #[arg(long = "password")]
        password: String,

        /// Input is Base64 encoded
        #[arg(long = "base64")]
        base64: bool,
    }
}

fn main() {
    let args = Args::parse();

   let result = match args.command {
        Commands::Encrypt { input, output, method, password, base64 } => handle_encrypt(
            &input,
            output.as_ref(),
            &method,
            &password,
            base64,
        ),
         Commands::Decrypt { input, output, method, password, base64 } => handle_decrypt(
            &input,
            output.as_ref(),
            &method,
            &password,
            base64,
        ),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn handle_encrypt(
    input: &str,
    output: Option<&String>,
    method: &CryptoMethod,
    password: &str,
    use_base64: bool,
) -> Result<()> {
    println!("Encrypting file '{}' using {:?}...", input, method);

    let data = read_input_file(input)?;
    let encrypted = encrypt_data(&data, method, password)?;

    let final_data = if use_base64 {
        general_purpose::STANDARD.encode(&encrypted).into_bytes()
    } else {
        encrypted
    };

    write_output(&final_data, output)?;

    if output.is_some() {
        println!("Encryption completed successfully!");
    }

    Ok(())
}

fn handle_decrypt(
    input: &str,
    output: Option<&String>,
    method: &CryptoMethod,
    password: &str,
    use_base64: bool,
) -> Result<()> {
    println!("Decrypting file '{}' using {:?}...", input, method);

    let mut data = read_input_file(input)?;

    if use_base64 {
        let decoded = general_purpose::STANDARD.decode(&data)?;
        data = decoded;
    }

    let decrypted = decrypt_data(&data, method, password)?;

    write_output(&decrypted, output)?;

    if output.is_some() {
        println!("Decryption completed successfully!");
    }

    Ok(())
}