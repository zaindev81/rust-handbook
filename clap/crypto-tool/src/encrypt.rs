use crate::types::{CryptoMethod, CryptoError, Result};

// Crypto dependencies (add these to Cargo.toml)
use aes::Aes256;
use aes::cipher::{
    BlockEncrypt, BlockDecrypt, KeyInit,
    generic_array::GenericArray,
};
use des::Des;
use rsa::{RsaPrivateKey, RsaPublicKey, Pkcs1v15Encrypt};
use rand::rngs::OsRng;
use sha2::{Sha256, Digest};

pub fn encrypt_data(data: &[u8], method: &CryptoMethod, password: &str) -> Result<Vec<u8>> {
    match method {
        CryptoMethod::Aes => aes_encrypt(data, password),
        CryptoMethod::Des => des_encrypt(data, password),
        CryptoMethod::Rsa => rsa_encrypt(data, password),
    }
}

pub fn decrypt_data(data: &[u8], method: &CryptoMethod, password: &str) -> Result<Vec<u8>> {
    match method {
        CryptoMethod::Aes => aes_decrypt(data, password),
        CryptoMethod::Des => des_decrypt(data, password),
        CryptoMethod::Rsa => rsa_decrypt(data, password),
    }
}

fn derive_key(password: &str, key_size: usize) -> Vec<u8>{
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    let result = hasher.finalize();

    // Truncate or pad to desired key size
    let mut key = vec![0u8; key_size];
    let copy_len = std::cmp::min(result.len(), key_size);
    key[..copy_len].copy_from_slice(&result[..copy_len]);
    key
}

fn aes_encrypt(data: &[u8], password: &str) -> Result<Vec<u8>> {
    let key = derive_key(password, 32); // AES-256 key size
    let key_array = GenericArray::from_slice(&key);
    let cipher = Aes256::new(key_array);

    // Pad data to 16-byte block size
    let mut padded_data = data.to_vec();
    let padding_needed = 16 - (data.len() % 16);
    padded_data.extend(vec![padding_needed as u8; padding_needed]);


    let mut encrypted = Vec::new();
    for chunk in padded_data.chunks(16) {
        let mut block = GenericArray::clone_from_slice(chunk);
        cipher.encrypt_block(&mut block);
        encrypted.extend_from_slice(&block);
    }

    Ok(encrypted)
}

fn aes_decrypt(data: &[u8], password: &str) -> Result<Vec<u8>> {
    let key = derive_key(password, 32); // AES-256
    let key_array = GenericArray::from_slice(&key);
    let cipher = Aes256::new(key_array);

    if data.len() % 16 != 0 {
        return Err(CryptoError::InvalidDataLength);
    }

    let mut decrypted = Vec::new();
    for chunk in data.chunks(16) {
        let mut block = GenericArray::clone_from_slice(chunk);
        cipher.decrypt_block(&mut block);
        decrypted.extend_from_slice(&block);
    }

    // Remove padding
    if let Some(&padding) = decrypted.last() {
        let padding_len = padding as usize;
        if padding_len > 0 && padding_len <= 16 && padding_len <= decrypted.len() {
            decrypted.truncate(decrypted.len() - padding_len);
        }
    }

    Ok(decrypted)
}

// DES encryption/decryption
fn des_encrypt(data: &[u8], password: &str) -> Result<Vec<u8>> {
    let key = derive_key(password, 8); // DES key is 8 bytes
    let key_array = GenericArray::from_slice(&key);
    let cipher = Des::new(key_array);

    // Pad data to 8-byte blocks
    let mut padded_data = data.to_vec();
    let padding_needed = 8 - (data.len() % 8);
    padded_data.extend(vec![padding_needed as u8; padding_needed]);

    let mut encrypted = Vec::new();
    for chunk in padded_data.chunks(8) {
        let mut block = GenericArray::clone_from_slice(chunk);
        cipher.encrypt_block(&mut block);
        encrypted.extend_from_slice(&block);
    }

    Ok(encrypted)
}

fn des_decrypt(data: &[u8], password: &str) -> Result<Vec<u8>> {
    let key = derive_key(password, 8); // DES key is 8 bytes
    let key_array = GenericArray::from_slice(&key);
    let cipher = Des::new(key_array);

    if data.len() % 8 != 0 {
        return Err(CryptoError::InvalidDataLength);
    }

    let mut decrypted = Vec::new();
    for chunk in data.chunks(8) {
        let mut block = GenericArray::clone_from_slice(chunk);
        cipher.decrypt_block(&mut block);
        decrypted.extend_from_slice(&block);
    }

    // Remove padding
    if let Some(&padding) = decrypted.last() {
        let padding_len = padding as usize;
        if padding_len > 0 && padding_len <= 8 && padding_len <= decrypted.len() {
            decrypted.truncate(decrypted.len() - padding_len);
        }
    }

    Ok(decrypted)
}

// RSA encryption/decryption
fn rsa_encrypt(data: &[u8], _password: &str) -> Result<Vec<u8>> {
    // For RSA, we'll generate a key pair (in a real implementation, you'd save/load keys)
    let mut rng = OsRng;
    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut rng, bits)
        .map_err(|e| CryptoError::CryptoError(e.to_string()))?;
    let public_key = RsaPublicKey::from(&private_key);

    // RSA can only encrypt small amounts of data
    // For larger data, you'd typically use hybrid encryption
    if data.len() > 245 { // Conservative limit for 2048-bit RSA with PKCS1v15 padding
        return Err(CryptoError::CryptoError(
            "Data too large for RSA encryption. Use AES for large files.".to_string()
        ));
    }

    let encrypted = public_key.encrypt(&mut rng, Pkcs1v15Encrypt, data)
        .map_err(|e| CryptoError::CryptoError(e.to_string()))?;

    Ok(encrypted)
}

fn rsa_decrypt(data: &[u8], _password: &str) -> Result<Vec<u8>> {
    // This is a simplified example - in practice, you'd load the private key
    // For this demo, RSA decrypt won't work without the original private key
    Err(CryptoError::CryptoError(
        "RSA decryption requires the original private key. This is a demo limitation.".to_string()
    ))
}
