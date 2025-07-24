use std::fs;
use std::io::{self, Write};
use std::path::Path;

use crate::types::{CryptoError, Result};

pub fn read_input_file(path: &str) -> Result<Vec<u8>> {
    if !Path::new(path).exists() {
        return Err(CryptoError::IoError(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Input file '{}' not found", path)
        )));
    }

    fs::read(path).map_err(CryptoError::from)
}

pub fn write_output(data: &[u8], output_path: Option<&String>) -> Result<()> {
    match output_path {
        Some(path) => {
            fs::write(path, data)?;
            println!("Output written to: {}", path);
        }
        None => {
            io::stdout().write_all(data)?;
            io::stdout().flush()?;
        }
    }
    Ok(())
}