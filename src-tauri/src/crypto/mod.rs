//! Cryptographic primitives for ChronoXP.
//!
//! # Key hierarchy
//! - **Master key (MK)**: 32 random bytes. Root secret that encrypts journal content.
//! - **Keychain storage**: MK stored via OS keychain (`keyring`). No vault password required.
//! - **Vault password path**: Argon2id(password, salt) → wrap_key → AES-256-GCM(MK) stored in
//!   settings table. MK never stored in plaintext in the DB.
//! - **Recovery key**: base32(MK) shown once on first setup, never stored.
//!
//! # Entry content encryption
//! Each `markdown` value is stored as:
//!   base64(nonce(12 bytes) || AES-256-GCM-ciphertext)

use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit},
    Aes256Gcm, Key, Nonce,
};
use argon2::{Algorithm, Argon2, Params, Version};
use base64::{engine::general_purpose::STANDARD as B64, Engine};
use pbkdf2::pbkdf2_hmac;
use rand_core::{OsRng, RngCore};
use sha2::Sha256;
use thiserror::Error;
use zeroize::Zeroize;

pub const KEYRING_SERVICE: &str = "chronoxp_journal";
pub const KEYRING_USER: &str = "master_key_v1";

/// Errors produced by crypto operations.
#[derive(Debug, Error)]
pub enum CryptoError {
    #[error("Decryption failed — wrong key or corrupted data")]
    DecryptionFailed,
    #[error("Encryption failed")]
    EncryptionFailed,
    #[error("Key derivation failed: {0}")]
    KdfFailed(String),
    #[error("Invalid ciphertext format")]
    InvalidFormat,
    #[error("Keychain error: {0}")]
    KeychainError(String),
}

/// Generate a fresh random 32-byte master key.
pub fn generate_master_key() -> [u8; 32] {
    let mut key = [0u8; 32];
    OsRng.fill_bytes(&mut key);
    key
}

/// Encode master key as base32 (recovery key shown to user once).
pub fn encode_recovery_key(key: &[u8; 32]) -> String {
    base32::encode(base32::Alphabet::RFC4648 { padding: false }, key)
}

/// Decode a recovery key back to raw bytes.
pub fn decode_recovery_key(encoded: &str) -> Option<[u8; 32]> {
    let bytes = base32::decode(base32::Alphabet::RFC4648 { padding: false }, encoded)?;
    if bytes.len() != 32 {
        return None;
    }
    let mut arr = [0u8; 32];
    arr.copy_from_slice(&bytes);
    Some(arr)
}

/// Derive a 32-byte wrap key from a password using Argon2id.
/// Returns `(derived_key, salt_hex)`.
pub fn derive_wrap_key(password: &str, salt_hex: Option<&str>) -> Result<([u8; 32], String), CryptoError> {
    let salt: [u8; 16] = match salt_hex {
        Some(hex_str) => {
            let bytes = hex::decode(hex_str).map_err(|_| CryptoError::InvalidFormat)?;
            if bytes.len() != 16 {
                return Err(CryptoError::InvalidFormat);
            }
            let mut arr = [0u8; 16];
            arr.copy_from_slice(&bytes);
            arr
        }
        None => {
            let mut s = [0u8; 16];
            OsRng.fill_bytes(&mut s);
            s
        }
    };

    let params = Params::new(65536, 3, 1, Some(32))
        .map_err(|e| CryptoError::KdfFailed(e.to_string()))?;
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

    let mut output = [0u8; 32];
    argon2
        .hash_password_into(password.as_bytes(), &salt, &mut output)
        .map_err(|e| CryptoError::KdfFailed(e.to_string()))?;

    Ok((output, hex::encode(salt)))
}

/// Wrap (encrypt) the master key with a wrap key using AES-256-GCM.
/// Returns base64(nonce || ciphertext).
pub fn wrap_master_key(master_key: &[u8; 32], wrap_key: &[u8; 32]) -> Result<String, CryptoError> {
    encrypt(master_key, wrap_key)
}

/// Unwrap (decrypt) the master key with a wrap key.
pub fn unwrap_master_key(wrapped_b64: &str, wrap_key: &[u8; 32]) -> Result<[u8; 32], CryptoError> {
    let bytes = decrypt(wrapped_b64, wrap_key)?;
    if bytes.len() != 32 {
        return Err(CryptoError::DecryptionFailed);
    }
    let mut arr = [0u8; 32];
    arr.copy_from_slice(&bytes);
    Ok(arr)
}

/// Encrypt plaintext bytes with a 32-byte AES-256-GCM key.
/// Returns base64(nonce(12) || ciphertext).
pub fn encrypt(plaintext: &[u8], key: &[u8; 32]) -> Result<String, CryptoError> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = cipher
        .encrypt(&nonce, plaintext)
        .map_err(|_| CryptoError::EncryptionFailed)?;
    let mut combined = Vec::with_capacity(12 + ciphertext.len());
    combined.extend_from_slice(&nonce);
    combined.extend_from_slice(&ciphertext);
    Ok(B64.encode(&combined))
}

/// Decrypt a value produced by `encrypt`. Returns plaintext bytes.
pub fn decrypt(ciphertext_b64: &str, key: &[u8; 32]) -> Result<Vec<u8>, CryptoError> {
    let combined = B64.decode(ciphertext_b64).map_err(|_| CryptoError::InvalidFormat)?;
    if combined.len() < 12 {
        return Err(CryptoError::InvalidFormat);
    }
    let (nonce_bytes, ciphertext) = combined.split_at(12);
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    let nonce = Nonce::from_slice(nonce_bytes);
    cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| CryptoError::DecryptionFailed)
}

/// Encrypt a UTF-8 string and return base64 ciphertext.
pub fn encrypt_str(plaintext: &str, key: &[u8; 32]) -> Result<String, CryptoError> {
    encrypt(plaintext.as_bytes(), key)
}

/// Decrypt a base64 ciphertext and return a UTF-8 string.
pub fn decrypt_str(ciphertext_b64: &str, key: &[u8; 32]) -> Result<String, CryptoError> {
    let bytes = decrypt(ciphertext_b64, key)?;
    String::from_utf8(bytes).map_err(|_| CryptoError::DecryptionFailed)
}

/// Derive an export key from a user-supplied password using PBKDF2-SHA256.
pub fn derive_export_key(password: &str, salt: &[u8]) -> [u8; 32] {
    let mut key = [0u8; 32];
    pbkdf2_hmac::<Sha256>(password.as_bytes(), salt, 100_000, &mut key);
    key
}

/// Store the master key in the OS keychain.
pub fn store_in_keychain(key: &[u8; 32]) -> Result<(), CryptoError> {
    let entry = keyring::Entry::new(KEYRING_SERVICE, KEYRING_USER)
        .map_err(|e| CryptoError::KeychainError(e.to_string()))?;
    let b64_key = B64.encode(key);
    entry
        .set_password(&b64_key)
        .map_err(|e| CryptoError::KeychainError(e.to_string()))?;
    Ok(())
}

/// Retrieve the master key from the OS keychain.
pub fn load_from_keychain() -> Result<[u8; 32], CryptoError> {
    let entry = keyring::Entry::new(KEYRING_SERVICE, KEYRING_USER)
        .map_err(|e| CryptoError::KeychainError(e.to_string()))?;
    let b64_key = entry
        .get_password()
        .map_err(|e| CryptoError::KeychainError(e.to_string()))?;
    let bytes = B64.decode(&b64_key).map_err(|_| CryptoError::InvalidFormat)?;
    if bytes.len() != 32 {
        return Err(CryptoError::InvalidFormat);
    }
    let mut arr = [0u8; 32];
    arr.copy_from_slice(&bytes);
    Ok(arr)
}

/// Zero out a key from memory.
pub fn zero_key(key: &mut [u8; 32]) {
    key.zeroize();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let key = generate_master_key();
        let plaintext = "Hello, encrypted world!";
        let ciphertext = encrypt_str(plaintext, &key).expect("encrypt failed");
        let decrypted = decrypt_str(&ciphertext, &key).expect("decrypt failed");
        assert_eq!(plaintext, decrypted);
    }

    #[test]
    fn test_wrong_key_fails() {
        let key = generate_master_key();
        let wrong_key = generate_master_key();
        let plaintext = "Secret journal entry";
        let ciphertext = encrypt_str(plaintext, &key).expect("encrypt failed");
        let result = decrypt_str(&ciphertext, &wrong_key);
        assert!(result.is_err(), "decryption with wrong key must fail");
        match result.unwrap_err() {
            CryptoError::DecryptionFailed => {}
            e => panic!("unexpected error: {e}"),
        }
    }

    #[test]
    fn test_recovery_key_encode_decode() {
        let key = generate_master_key();
        let encoded = encode_recovery_key(&key);
        let decoded = decode_recovery_key(&encoded).expect("decode failed");
        assert_eq!(key, decoded);
    }

    #[test]
    fn test_recovery_key_wrong_input() {
        assert!(decode_recovery_key("not-valid-base32!!!").is_none());
    }

    #[test]
    fn test_wrap_unwrap_master_key() {
        let master_key = generate_master_key();
        let wrap_key = generate_master_key();
        let wrapped = wrap_master_key(&master_key, &wrap_key).expect("wrap failed");
        let unwrapped = unwrap_master_key(&wrapped, &wrap_key).expect("unwrap failed");
        assert_eq!(master_key, unwrapped);
    }

    #[test]
    fn test_wrap_wrong_wrap_key_fails() {
        let master_key = generate_master_key();
        let wrap_key = generate_master_key();
        let wrong_wrap_key = generate_master_key();
        let wrapped = wrap_master_key(&master_key, &wrap_key).expect("wrap failed");
        let result = unwrap_master_key(&wrapped, &wrong_wrap_key);
        assert!(result.is_err());
    }

    #[test]
    fn test_derive_wrap_key_deterministic() {
        let password = "correct horse battery staple";
        let (key1, salt) = derive_wrap_key(password, None).expect("derive failed");
        let (key2, _) = derive_wrap_key(password, Some(&salt)).expect("derive with salt failed");
        assert_eq!(key1, key2, "same password+salt must produce same key");
    }

    #[test]
    fn test_derive_wrap_key_different_passwords() {
        let (key1, salt) = derive_wrap_key("password1", None).expect("derive1 failed");
        let (key2, _) = derive_wrap_key("password2", Some(&salt)).expect("derive2 failed");
        assert_ne!(key1, key2, "different passwords must produce different keys");
    }

    #[test]
    fn test_export_key_derivation() {
        let salt = b"test_salt_16byte";
        let key = derive_export_key("my_backup_password", salt);
        let key2 = derive_export_key("my_backup_password", salt);
        assert_eq!(key, key2, "same input must produce same export key");
        let key3 = derive_export_key("different_password", salt);
        assert_ne!(key, key3);
    }

    #[test]
    fn test_ciphertext_format_corrupted() {
        let key = generate_master_key();
        let result = decrypt_str("not-valid-base64!!!", &key);
        assert!(result.is_err());
    }

    #[test]
    fn test_ciphertext_too_short() {
        let key = generate_master_key();
        let result = decrypt_str("AAAA", &key);
        assert!(result.is_err());
    }

    #[test]
    fn test_encrypt_empty_string() {
        let key = generate_master_key();
        let ciphertext = encrypt_str("", &key).expect("encrypt empty failed");
        let decrypted = decrypt_str(&ciphertext, &key).expect("decrypt empty failed");
        assert_eq!("", decrypted);
    }

    #[test]
    fn test_encrypt_large_content() {
        let key = generate_master_key();
        let large = "A".repeat(100_000);
        let ciphertext = encrypt_str(&large, &key).expect("encrypt large failed");
        let decrypted = decrypt_str(&ciphertext, &key).expect("decrypt large failed");
        assert_eq!(large, decrypted);
    }
}
