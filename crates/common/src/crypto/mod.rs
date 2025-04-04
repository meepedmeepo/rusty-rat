pub mod encryption;
mod identity_keys;

pub use identity_keys::*;

use blake2::{
    Blake2b, VarBlake2b,
    digest::{Update, VariableOutput},
};
use rand::{RngCore, rngs::OsRng};
use x25519_dalek::{PublicKey, StaticSecret};
const XCHACHA20_POLY1305_KEY_SIZE: usize = 32;
pub const XCHACHA20_POLY1305_NONCE_SIZE: usize = 24;

///Private and public keypair to be used for diffie-hellman
struct Keypair {
    priv_key: StaticSecret,
    pub_key: PublicKey,
}

impl Keypair {
    pub fn new() -> Self {
        let random_generator = OsRng {};

        let priv_key = StaticSecret::random_from_rng(random_generator);
        let pub_key = PublicKey::from(&priv_key);

        Keypair { priv_key, pub_key }
    }

    pub fn get_public_key(&self) -> PublicKey {
        self.pub_key
    }

    ///Creates shared secret that to be used as encryption key for message
    pub fn generate_secret(
        &self,
        recieved_public_key: PublicKey,
        nonce: &[u8; XCHACHA20_POLY1305_NONCE_SIZE],
    ) -> Vec<u8> {
        let dh_secret = self.priv_key.diffie_hellman(&recieved_public_key);

        let mut kdf =
            blake2::VarBlake2b::new_keyed(dh_secret.as_bytes(), XCHACHA20_POLY1305_KEY_SIZE);
        VarBlake2b::update(&mut kdf, nonce);

        let shared_key = kdf.finalize_boxed();

        shared_key.into()
    }
}

pub fn gen_diffie_hellman_keys() {
    let mut random_generator = OsRng {};

    let mut nonce = [0u8; XCHACHA20_POLY1305_NONCE_SIZE];
    random_generator.fill_bytes(&mut nonce);

    let priv_key = StaticSecret::random_from_rng(random_generator);
    let pub_key = PublicKey::from(&priv_key);
}
