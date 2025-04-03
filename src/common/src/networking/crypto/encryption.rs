use chacha20poly1305::{Error, KeyInit, XChaCha20Poly1305, aead::Aead};
use thiserror::Error;
use x25519_dalek::PublicKey;

use super::{XCHACHA20_POLY1305_KEY_SIZE, XCHACHA20_POLY1305_NONCE_SIZE};

#[derive(Debug, PartialEq, Error)]
pub enum CryptographyError {
    #[error("Encryption failed to complete: {}", .0)]
    EncryptionFailed(Error),

    #[error("Decryption failed to complete: {}", .0)]
    DecryptionFailed(Error),
}

pub fn encrypt_XCha20ChaPoly1305(
    nonce: [u8; XCHACHA20_POLY1305_NONCE_SIZE],
    encryption_key: [u8; XCHACHA20_POLY1305_KEY_SIZE],
    plain_text: &[u8],
) -> Result<Vec<u8>, CryptographyError> {
    let cipher = XChaCha20Poly1305::new(&encryption_key.into());

    let cipher_text = cipher
        .encrypt(&nonce.into(), plain_text)
        .map_err(CryptographyError::EncryptionFailed)?;

    Ok(cipher_text)
}

pub fn decrypt_XCha20ChaPoly1305(
    nonce: [u8; XCHACHA20_POLY1305_NONCE_SIZE],
    decryption_key: [u8; XCHACHA20_POLY1305_KEY_SIZE],
    cipher_text: &[u8],
) -> Result<Vec<u8>, CryptographyError> {
    let decoder = XChaCha20Poly1305::new(&decryption_key.into());

    let plain_text = decoder
        .decrypt(&nonce.into(), cipher_text)
        .map_err(CryptographyError::DecryptionFailed)?;

    Ok(plain_text)
}
