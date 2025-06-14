use std::fs;
use aes_gcm::{Aes256Gcm, Nonce};
use aes_gcm::aead::{Aead, KeyInit};
use sha2::{Digest, Sha256};

pub mod crypto {
    pub use super::{encrypt_file, decrypt_file};
}

pub fn encrypt_file(file_path: &str, output_path: &str, password: String) -> Result<bool, String> {
    // Read the input file
    let data_bytes = fs::read(file_path)
        .map_err(|e| format!("Failed to read input file: {}", e))?;

    // Generate a 32-byte key from the password using SHA-256
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    let key_bytes = hasher.finalize();

    // Create AES-256-GCM cipher from the key
    let cipher = Aes256Gcm::new_from_slice(&key_bytes)
        .map_err(|e| format!("Key creation error: {:?}", e))?;

    // Create a 12-byte nonce. AES-GCM requires a 12-byte (96-bit) nonce.
    let nonce = Nonce::from_slice(b"unique_nonce"); // Exactly 12 bytes

    // Encrypt the data
    let encrypted_data = cipher
        .encrypt(nonce, data_bytes.as_ref())
        .map_err(|e| format!("Encryption error: {:?}", e))?;

    // Write the encrypted data to the output file
    fs::write(output_path, encrypted_data)
        .map_err(|e| format!("Failed to write encrypted file: {}", e))?;

    Ok(true)
}

pub fn decrypt_file(encrypted_file_path: &str, output_path: &str, password: String) -> Result<bool, String> {
    // Read the encrypted file
    let encrypted_data = fs::read(encrypted_file_path)
        .map_err(|e| format!("Failed to read encrypted file: {}", e))?;

    // Derive the key from the provided password using SHA-256
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    let key_bytes = hasher.finalize();

    // Create the AES-256-GCM cipher for decryption from the derived key
    let cipher = Aes256Gcm::new_from_slice(&key_bytes)
        .map_err(|e| format!("Key creation error: {:?}", e))?;

    // Use the same fixed 12-byte nonce as in encryption
    let nonce = Nonce::from_slice(b"unique_nonce");

    // Attempt to decrypt the data
    let decrypted_data = cipher
        .decrypt(nonce, encrypted_data.as_ref())
        .map_err(|e| format!("Decryption error: {:?}", e))?;

    // Write the decrypted data to the output file
    fs::write(output_path, decrypted_data)
        .map_err(|e| format!("Failed to write decrypted file: {}", e))?;

    Ok(true)
}
