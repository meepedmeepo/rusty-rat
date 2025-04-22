use anyhow::{Error, anyhow};
use chrono::Utc;
use common::schemas::{AgentRegistered, RegisterAgent};
use uuid::Uuid;

use crate::services::Agent;

use super::{Service, entities};

impl Service {
    pub async fn list_agents(&self) -> Result<Vec<entities::Agent>, Error> {
        self.repo.get_all_agents(&self.db).await
    }

    pub async fn find_agent(&self, agent_id: Uuid) -> Result<entities::Agent, Error> {
        self.repo.find_agent_by_id(&self.db, agent_id).await
    }

    ///Verifies that the signature of the prekey matches the prekey so there has been no MITM tampering
    ///, and then if it is valid it creates a row for a new agent
    pub async fn register_agent(&self, input: RegisterAgent) -> Result<AgentRegistered, Error> {
        let id = Uuid::new_v4();
        let created_at = Utc::now();

        //TODO! check if the size of the prekey signature is valid

        let identity_public_key =
            ed25519_dalek::VerifyingKey::from_bytes(&input.public_identity_key)?;

        let signature = ed25519_dalek::Signature::try_from(&input.public_prekey_signature[0..64])?;

        if identity_public_key
            .verify_strict(&input.public_prekey, &signature)
            .is_err()
        {
            return Err(anyhow!("Agent signature is not valid!"));
        }

        let agent = Agent {
            id,
            created_at,
            last_seen_at: created_at,
            identity_public_key: input.public_identity_key.to_vec(),
            public_prekey: input.public_prekey.to_vec(),
            public_prekey_signature: input.public_prekey_signature,
        };

        self.repo.register_agent(&self.db, agent).await?;

        Ok(AgentRegistered { id })
    }
}
