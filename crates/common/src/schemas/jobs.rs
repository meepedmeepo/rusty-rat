use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::cryptographic_functions::{X25519_KEY_SIZE, XCHACHA20_POLY1305_NONCE_SIZE};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateJob {
    pub id: Uuid,
    pub agent_id: Uuid,
    pub encrypted_job: Vec<u8>,
    pub ephemeral_public_key: [u8; X25519_KEY_SIZE],
    pub nonce: [u8; XCHACHA20_POLY1305_NONCE_SIZE],
    pub signature: Vec<u8>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JobPayload {
    pub command: String,
    pub args: Vec<String>,
    pub result_ephemeral_public_key: [u8; X25519_KEY_SIZE],
}
