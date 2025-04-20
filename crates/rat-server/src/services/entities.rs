use chrono::{DateTime, Utc};
use uuid::Uuid;

///This struct purely exists to deserialize from postgres database results without having to reason about lifetimes of
/// and borrowing of arrays whilst still being able to have static assurances of the integrity of the data by try_into())
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Job {
    pub id: Uuid,
    pub agent_id: Uuid,
    pub encrypted_job: Vec<u8>,
    pub ephemeral_public_key: Vec<u8>,
    pub nonce: Vec<u8>,
    pub signature: Vec<u8>,
    pub encrypted_result: Option<Vec<u8>>,
    pub result_ephemeral_public_key: Option<Vec<u8>>,
    pub result_nonce: Option<Vec<u8>>,
    pub result_signature: Option<Vec<u8>>,
}

impl Into<common::schemas::Job> for Job {
    fn into(self) -> common::schemas::Job {
        common::schemas::Job {
            id: self.id,
            agent_id: self.agent_id,
            encrypted_job: self.encrypted_job,
            ephemeral_public_key: self
                .ephemeral_public_key
                .try_into()
                .expect("ephemeral_public_key is invalid converting Job to schema::Job"),
            nonce: self
                .nonce
                .try_into()
                .expect("nonce is invalid converting Job to schema::Job"),
            signature: self.signature,
            encrypted_result: self.encrypted_result,
            result_ephemeral_public_key: self.result_ephemeral_public_key.map(|v| {
                v.try_into()
                    .expect("result_ephemeral_public_key is invalid converting Job to schema::Job")
            }),
            result_nonce: self.result_nonce.map(|v| {
                v.try_into()
                    .expect("result_nonce is invalid converting to schema::Job")
            }),
            result_signature: self.result_signature,
        }
    }
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Agent {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub last_seen_at: DateTime<Utc>,
    pub identity_public_key: Vec<u8>,
    pub public_prekey: Vec<u8>,
    pub public_prekey_signature: Vec<u8>,
}

impl Into<common::schemas::Agent> for Agent {
    fn into(self) -> common::schemas::Agent {
        common::schemas::Agent {
            id: self.id,
            created_at: self.created_at,
            last_seen_at: self.last_seen_at,
            identity_public_key: self
                .identity_public_key
                .try_into()
                .expect("identity_public_key is invalid converting to schema::Agent"),
            public_prekey: self
                .public_prekey
                .try_into()
                .expect("public_prekey is invalid converting to schema::Agent"),
            public_prekey_signature: self.public_prekey_signature,
        }
    }
}
