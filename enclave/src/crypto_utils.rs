use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm,
    Key, // Or `Aes128Gcm`
    Nonce,
};
use base64::{engine::general_purpose, Engine as _};

pub fn encrypt_by_data_key(data_key_plaintext_base64: &str, private_key: &str) -> String {
    let data_key_bytes = general_purpose::STANDARD
        .decode(data_key_plaintext_base64)
        .expect("Input file does not contain valid base 64 characters.");

    // Create a key for AES256
    let key = Key::<Aes256Gcm>::from_slice(&data_key_bytes);

    // Create a new AES256 cipher
    let cipher = Aes256Gcm::new(key);

    // let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per message
    let nonce_bytes = [204, 92, 172, 44, 119, 145, 175, 178, 245, 248, 89, 193];
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher.encrypt(nonce, private_key.as_bytes()).unwrap();
    let ciphertext_base64 = general_purpose::STANDARD.encode(ciphertext);
    ciphertext_base64
}