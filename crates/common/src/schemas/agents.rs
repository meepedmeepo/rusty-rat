use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::cryptographic_functions::X25519_KEY_SIZE;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Agent {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub last_seen_at: DateTime<Utc>,
    pub identity_public_key: [u8; ed25519_dalek::PUBLIC_KEY_LENGTH],
    pub public_prekey: [u8; X25519_KEY_SIZE],
    pub public_prekey_signature: Vec<u8>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AgentsList {
    pub agents: Vec<Agent>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AgentRegistered {
    pub id: Uuid,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RegisterAgent {
    pub public_identity_key: [u8; ed25519_dalek::PUBLIC_KEY_LENGTH],
    pub public_prekey: [u8; X25519_KEY_SIZE],
    pub public_prekey_signature: Vec<u8>,
}
