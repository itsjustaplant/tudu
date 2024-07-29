use hex::{decode, encode};
use magic_crypt::new_magic_crypt;
use magic_crypt::MagicCryptTrait;

pub fn encrypt(data: &str, key: &str) -> String {
    let mc = new_magic_crypt!(key, 256);
    let encrypted = mc.encrypt_str_to_bytes(data);
    encode(encrypted)
}

pub fn decrypt(encrypted_data: &str, key: &str) -> Result<String, Box<dyn std::error::Error>> {
    let trimmed_data = &encrypted_data[1..encrypted_data.len() - 1];
    let mc = new_magic_crypt!(key, 256);
    let encrypted_bytes = decode(trimmed_data)?;
    let decrypted_bytes = mc.decrypt_bytes_to_bytes(&encrypted_bytes)?;
    Ok(String::from_utf8(decrypted_bytes)?)
}
