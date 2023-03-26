use chacha20poly1305::{aead::Aead, ChaCha20Poly1305, KeyInit};

use base64::{engine::general_purpose::URL_SAFE, Engine as _};

use argon2::{
    password_hash::rand_core::{OsRng, RngCore},
    Argon2,
};

fn create_salt() -> [u8; 16] {
    let mut salt = [0u8; 16];
    (&mut OsRng).fill_bytes(&mut salt);

    salt
}

fn hash_password(password: &str, salt: &[u8; 16]) -> [u8; 32] {
    let mut hash = [0u8; 32];
    Argon2::default()
        .hash_password_into(password.as_bytes(), salt, &mut hash)
        .expect("Argon2 failure!");

    hash
}

fn encrypt_text(text: &str, key: &[u8; 32]) -> (Vec<u8>, [u8; 12]) {
    let aead = ChaCha20Poly1305::new(key.into());

    let mut nonce = [0u8; 12];
    (&mut OsRng).fill_bytes(&mut nonce);

    let encrypted_text = aead
        .encrypt(&nonce.into(), text.as_bytes())
        .expect("Encryption failure!");

    (encrypted_text, nonce)
}

fn decrypt_text(encrypted_text: &[u8], key: &[u8; 32], nonce: &[u8; 12]) -> String {
    let aead = ChaCha20Poly1305::new(key.into());

    let decrypted_text = aead
        .decrypt(nonce.into(), encrypted_text)
        .expect("Decryption failure!");
    String::from_utf8(decrypted_text).unwrap()
}

fn encode_payload(salt: &[u8; 16], nonce: &[u8; 12], encrypted_text: &[u8]) -> String {
    format!(
        "{}+{}+{}",
        URL_SAFE.encode(salt),
        URL_SAFE.encode(nonce),
        URL_SAFE.encode(encrypted_text)
    )
}

fn decode_payload(payload: &str) -> ([u8; 16], [u8; 12], Vec<u8>) {
    let mut split = payload.split_terminator('+');
    let mut salt = [0u8; 16];
    let mut nonce = [0u8; 12];
    let _ = URL_SAFE
        .decode_slice_unchecked(split.next().unwrap(), &mut salt)
        .expect("Cant parse salt");
    let _ = URL_SAFE
        .decode_slice_unchecked(split.next().unwrap(), &mut nonce)
        .expect("Cant parse nonce");
    let encrypted_text = URL_SAFE
        .decode(split.next().unwrap())
        .expect("cant parse text");

    (salt, nonce, encrypted_text)
}

pub fn create_encoded_payload(password: &str, text: &str) -> String {
    let salt = create_salt();
    let key = hash_password(password, &salt);
    let (encrypted_text, nonce) = encrypt_text(text, &key);

    encode_payload(&salt, &nonce, &encrypted_text)
}

pub fn decrypt_encoded_payload(password: &str, payload: &str) -> String {
    let (salt, nonce, encrypted_text) = decode_payload(payload);
    let key = hash_password(password, &salt);

    decrypt_text(&encrypted_text, &key, &nonce)
}
