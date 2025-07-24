use std::fs;
use std::io;
use base64::{engine::general_purpose, Engine as _};

// CustomError
#[derive(Debug)]
enum MyError {
    Io(io::Error), // io
    Decode(base64::DecodeError), // decode
    EmptyInput // file is empty
}

// for ? debug
impl From<io::Error> for MyError {
    fn from(err: io::Error) -> Self {
        MyError::Io(err)
    }
}

impl From<base64::DecodeError> for MyError {
    fn from(err: base64::DecodeError) -> Self {
        MyError::Decode(err)
    }
}

// Display format
impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MyError::Io(e) => write!(f, "File Error {}", e),
            MyError::Decode(e) => write!(f, "Decode Error {}", e),
            MyError::EmptyInput => write!(f, "File is empty"),
        }
    }
}

impl std::error::Error for MyError {}

fn read_and_decode_file(path: &str) -> Result<Vec<u8>, MyError> {
    let contents = fs::read_to_string(path)?;
    if contents.trim().is_empty() {
        return Err(MyError::EmptyInput);
    }
    let decoded = general_purpose::STANDARD.decode(contents.trim())?;
    Ok(decoded)
}

pub fn custom_error() {
    match read_and_decode_file("src/err_module.rs") {
        Ok(data) => println!("Decoded data: {:?}", data),
        Err(e) => eprintln!("Error occurred: {}", e),
    }
}