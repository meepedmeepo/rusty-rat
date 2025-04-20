use ed25519_dalek::{SigningKey, VerifyingKey};
use std::sync::OnceLock;

pub static CLIENT_IDENTITY_PUBLIC_KEY: &str = include_str!("client_identity.secret"); //include!(std::fs::"client_identity.secret");
pub static AGENT_IDENTITY_SECRET: OnceLock<SigningKey> = OnceLock::new();
pub static AGENT_IDENTITY_PUBLIC: OnceLock<VerifyingKey> = OnceLock::new();
