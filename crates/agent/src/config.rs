use anyhow::Error;
use base64::Engine;
use ed25519_dalek::{SigningKey, VerifyingKey};
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;
use x25519_dalek::{X25519_BASEPOINT_BYTES, x25519};

pub static CLIENT_IDENTITY_PUBLIC_KEY: &str = include_str!("client_identity.secret"); //include!(std::fs::"client_identity.secret");
pub static SERVER_URL: &str = "http://localhost:8080";
//todo! probs have SERVER_URL as another include str possibly with encryption based off MAC address

#[derive(Debug)]
pub struct Config {
    pub agent_id: uuid::Uuid,
    pub identity_public_key: ed25519_dalek::VerifyingKey,
    pub identity_secret_key: ed25519_dalek::SigningKey,
    pub public_prekey: [u8; 32],
    pub private_prekey: [u8; 32],
    pub client_identity_public_key: ed25519_dalek::VerifyingKey,
}

impl Into<SerializedConfig> for Config {
    fn into(self) -> SerializedConfig {
        SerializedConfig {
            agent_id: self.agent_id,
            identity_private_key: self.identity_secret_key.to_bytes(),
            private_prekey: self.private_prekey,
        }
    }
}

impl TryFrom<SerializedConfig> for Config {
    type Error = Error;
    fn try_from(value: SerializedConfig) -> Result<Self, Self::Error> {
        let agent_id = value.agent_id;
        let identity_secret_key =
            ed25519_dalek::SigningKey::from_bytes(&value.identity_private_key);
        let identity_public_key: ed25519_dalek::VerifyingKey = (&identity_secret_key).into();

        let private_prekey = value.private_prekey;
        let public_prekey = x25519(private_prekey.clone(), X25519_BASEPOINT_BYTES);

        let client_public_key_bytes =
            base64::engine::general_purpose::STANDARD.decode(CLIENT_IDENTITY_PUBLIC_KEY)?;
        let client_identity_public_key = ed25519_dalek::VerifyingKey::from_bytes(
            &client_public_key_bytes.try_into().unwrap_or_default(),
        )?;

        Ok(Config {
            agent_id,
            client_identity_public_key,
            identity_secret_key,
            identity_public_key,
            private_prekey,
            public_prekey,
        })
    }
}

///THIS IS CONFIG THAT WILL BE SAVE TO DISK AS REST OF CONFIG CAN BE DERIVED FROM THIS (EXCEPT CLIENT IDENTITY KEY, BUT THAT
/// IS LOADED FROM DISK SEPERATELY BY THIS PROGRAM ANYWAY)
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SerializedConfig {
    pub agent_id: uuid::Uuid,
    pub identity_private_key: [u8; ed25519_dalek::SECRET_KEY_LENGTH],
    pub private_prekey: [u8; 32],
}
