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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_encryption_decryption() {
    let key = "SUPER_SECRET_KEY";
    let data = "Add more tests pls";

    let encrypted_data = encrypt(data, key);
    let decrypted_data = decrypt(format!("[{}]", encrypted_data).as_str(), key).expect("Decryption failed");

    assert_eq!(decrypted_data, data);
  }

  #[test]
  fn test_encryption_consistency() {
    let key = "SUPER_SECRET_KEY";
    let data = "Check consistency pls";

    let encrypted_data_0 = encrypt(data, key);
    let encrypted_data_1 = encrypt(data, key);

    assert_eq!(encrypted_data_0, encrypted_data_1);
  }

  #[test]
  fn test_decryption_invalid() {
    let key = "SUPER_SECRET_KEY";
    let encrypted_data = "Check invalid data pls";

    let decrypted_data = decrypt(encrypted_data, key);

    assert!(decrypted_data.is_err());
  }
}