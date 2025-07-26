use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Generate a random key 256 bits long
    let key = Aes256Gcm::generate_key(OsRng);
    let cipher = Aes256Gcm::new(&key);

    // Generate a random nonce (96-bits for GCM)
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let plaintext = b"Hello, cryptography!";

    // Encrypt the plaintext
    let ciphertext = cipher.encrypt(&nonce, plaintext.as_ref())
        .expect("encryption failure");
    println!("Plaintext: {}", String::from_utf8_lossy(plaintext));
    println!("Ciphertext: {:?}", ciphertext);

    // Decrypt the ciphertext
    let decrypted_plaintext = cipher.decrypt(&nonce, ciphertext.as_ref())
        .expect("decryption failure");

    println!("Decrypted Plaintext: {}", String::from_utf8_lossy(&decrypted_plaintext));

    // Ensure the decrypted plaintext matches the original plaintext
    assert_eq!(plaintext, &decrypted_plaintext[..]);
    println!("Decryption successful, plaintext matches original.");

    Ok(())
}