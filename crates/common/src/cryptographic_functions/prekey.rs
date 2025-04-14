use ed25519_dalek::{Signature, SigningKey, ed25519::signature::SignerMut};
use rand::RngCore;
use x25519_dalek::{PublicKey, StaticSecret, X25519_BASEPOINT_BYTES};

pub struct PrekeyPair {
    pub public_prekey: [u8; 32],
    pub private_prekey: [u8; 32],
}

impl PrekeyPair {
    pub fn new(public_prekey: [u8; 32], private_prekey: [u8; 32]) -> Self {
        Self {
            public_prekey,
            private_prekey,
        }
    }
}

pub fn create_and_sign_prekey(priv_identity_key: &SigningKey) -> (PrekeyPair, Signature) {
    let mut rand_generator = rand::rngs::OsRng {};
    let mut private_prekey = X25519_BASEPOINT_BYTES;

    rand_generator.fill_bytes(&mut private_prekey);

    let public_prekey = x25519_dalek::x25519(private_prekey.clone(), X25519_BASEPOINT_BYTES);

    let prekey_signature = priv_identity_key.clone().sign(&public_prekey);

    (
        PrekeyPair::new(public_prekey, private_prekey),
        prekey_signature,
    )
}
