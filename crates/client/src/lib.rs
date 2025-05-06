mod api_calls;
mod config;
mod parser;

pub use api_calls::*;
pub use config::*;
pub use parser::*;
use ureq::config::Config;

use std::{fs, time::Duration};

use base64::Engine;
pub use common;

pub fn init() -> ureq::Agent {
    ureq::Agent::new_with_config(
        Config::builder()
            .https_only(true)
            .timeout_global(Some(Duration::from_secs(10)))
            .build(),
    )
}

pub fn create_client_identity() {
    use base64::engine::general_purpose::STANDARD;

    let (priv_key, pub_key) = common::cryptographic_functions::create_identity();

    let priv_str = STANDARD.encode(priv_key.to_bytes());
    let pub_str = STANDARD.encode(pub_key.as_bytes());

    fs::write(
        "client_identity.secret",
        format!("{}\n{}\n", priv_str, pub_str),
    )
    .unwrap();
}
