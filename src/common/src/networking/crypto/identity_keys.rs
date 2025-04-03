use rand::rngs::OsRng;
use ed25519_dalek::*;



pub fn create_identity()
{
    let mut rand_generator = OsRng{};
    let identity_secret_key = SigningKey::generate(&mut rand_generator);
    let identity_public_key = identity_secret_key.verifying_key();
    //identity_public_key.

}