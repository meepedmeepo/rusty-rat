use serde::{Deserialize, Serialize};
use sqlx::FromRow;
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

#[derive(Debug, Clone, Deserialize, Serialize, FromRow)]
pub struct Job {
    pub id: Uuid,
    pub agent_id: Uuid,
    pub encrypted_job: Vec<u8>,
    pub ephemeral_public_key: [u8; X25519_KEY_SIZE],
    pub nonce: [u8; XCHACHA20_POLY1305_NONCE_SIZE],
    pub signature: Vec<u8>,
    pub encrypted_result: Option<Vec<u8>>,
    pub result_ephemeral_public_key: Option<[u8; X25519_KEY_SIZE]>,
    pub result_nonce: Option<[u8; XCHACHA20_POLY1305_NONCE_SIZE]>,
    pub result_signature: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AgentJob {
    pub id: Uuid,
    pub encrypted_job: Vec<u8>,
    pub ephemeral_public_key: [u8; X25519_KEY_SIZE],
    pub nonce: [u8; XCHACHA20_POLY1305_NONCE_SIZE],
    pub signature: Vec<u8>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JobResult {
    pub result: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpdateJobResult {
    pub job_id: Uuid,
    pub encrypted_job_result: Vec<u8>,
    pub ephemeral_public_key: [u8; X25519_KEY_SIZE],
    pub nonce: [u8; XCHACHA20_POLY1305_NONCE_SIZE],
    pub signature: Vec<u8>,
}
