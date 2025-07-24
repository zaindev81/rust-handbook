use std::io;

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum CryptoMethod {
    #[value(name = "aes")]
    Aes,
    #[value(name = "des")]
    Des,
    #[value(name = "rsa")]
    Rsa,
}

#[derive(Debug)]
pub enum CryptoError {
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

impl std::error::Error for CryptoError {}

pub type Result<T> = std::result::Result<T, CryptoError>;
