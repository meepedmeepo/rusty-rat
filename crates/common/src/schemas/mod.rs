mod agents;
mod jobs;
pub use agents::*;
use ed25519_dalek::{SIGNATURE_LENGTH, Signature, VerifyingKey};
pub use jobs::*;

use crate::cryptographic_functions::{
    XCHACHA20_POLY1305_KEY_SIZE, XCHACHA20_POLY1305_NONCE_SIZE, encryption::CryptographyError,
};

///This is a schema of what any message that has been encrypted will contain (only messages after public keys have been transferred)
pub struct EncryptedMessageSchema {
    ciphertext: Vec<u8>,
    nonce: [u8; XCHACHA20_POLY1305_NONCE_SIZE],
    public_key: [u8; XCHACHA20_POLY1305_KEY_SIZE],
    pub_identity_key: [u8; ed25519_dalek::PUBLIC_KEY_LENGTH],
    signature: [u8; SIGNATURE_LENGTH],
}

impl EncryptedMessageSchema {
    pub fn new(
        ciphertext: Vec<u8>,
        nonce: [u8; XCHACHA20_POLY1305_NONCE_SIZE],
        public_key: [u8; XCHACHA20_POLY1305_KEY_SIZE],
        pub_identity_key: [u8; ed25519_dalek::PUBLIC_KEY_LENGTH],
        signature: [u8; SIGNATURE_LENGTH],
    ) -> Self {
        EncryptedMessageSchema {
            ciphertext,
            nonce,
            public_key,
            pub_identity_key,
            signature,
        }
    }

    ///Verifies the authenticity of the digital signature of this message
    pub fn verify(&self) -> Result<(), CryptographyError> {
        let message = self
            .ciphertext
            .iter()
            .cloned()
            .chain(self.nonce.iter().cloned())
            .chain(self.public_key.iter().cloned())
            .chain(self.pub_identity_key.iter().cloned())
            .collect::<Vec<_>>();

        let identity_key = VerifyingKey::from_bytes(&self.pub_identity_key)
            .map_err(CryptographyError::SignatureInvalid)?;

        let signature = self.signature.into();

        match identity_key.verify_strict(&message, &signature) {
            Ok(_) => Ok(()),
            Err(err) => Err(CryptographyError::SignatureInvalid(err)),
        }
    }
}

pub struct HandshakeMessageSchema {
    nonce: [u8; XCHACHA20_POLY1305_NONCE_SIZE],
    public_key: [u8; XCHACHA20_POLY1305_KEY_SIZE],
    signature: [u8; SIGNATURE_LENGTH],
    pub_identity_key: [u8; ed25519_dalek::PUBLIC_KEY_LENGTH],
}

impl HandshakeMessageSchema {
    pub fn new(
        nonce: [u8; XCHACHA20_POLY1305_NONCE_SIZE],
        public_key: [u8; XCHACHA20_POLY1305_KEY_SIZE],
        signature: [u8; SIGNATURE_LENGTH],
        pub_identity_key: [u8; ed25519_dalek::PUBLIC_KEY_LENGTH],
    ) -> Self {
        HandshakeMessageSchema {
            nonce,
            public_key,
            signature,
            pub_identity_key,
        }
    }

    ///Tests if the message has a valid signature
    pub fn verify(&self) -> Result<(), CryptographyError> {
        let message = self
            .nonce
            .to_vec()
            .iter()
            .cloned()
            .chain(self.public_key.iter().cloned())
            .chain(self.pub_identity_key.iter().cloned())
            .collect::<Vec<_>>();

        let identity_key: VerifyingKey = VerifyingKey::from_bytes(&self.pub_identity_key)
            .map_err(CryptographyError::SignatureInvalid)?;

        let signature: Signature = self.signature.into();
        match identity_key.verify_strict(&message, &signature) {
            Ok(_) => return Ok(()),
            Err(err) => Err(CryptographyError::SignatureInvalid(err)),
        }
    }
}
