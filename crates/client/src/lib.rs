use std::fs;

use base64::Engine;
pub use common;

pub fn init()
{
    todo!();
}


pub fn create_client_identity()
{
    use base64::engine::general_purpose::STANDARD;

    let (priv_key, pub_key) = common::crypto::create_identity();

    let priv_str = STANDARD.encode(priv_key.to_bytes());
    let pub_str = STANDARD.encode(pub_key.as_bytes());

    fs::write("client_identity.secret", format!("{}\n{}\n",priv_str, pub_str)).unwrap();
}