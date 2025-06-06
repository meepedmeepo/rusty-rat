use ed25519_dalek::*;
use rand::rngs::OsRng;

pub fn create_identity() -> (SigningKey, VerifyingKey) {
    let mut rand_generator = OsRng {};
    let identity_secret_key = SigningKey::generate(&mut rand_generator);
    let identity_public_key = identity_secret_key.verifying_key();
    //ed25519_dalek::PUBLIC_KEY_LENGTH
    //identity_public_key.

    (identity_secret_key, identity_public_key)
}
