use anyhow::Error;
use common::schemas::{Agent, RegisterAgent};
use serde::Serialize;
use sqlx::{Pool, Postgres, query_as};
use uuid::Uuid;

use super::Repository;

impl Repository {
    ///Creates a row in the agents table for a newly created agent. This assumes that the validity of the record has already
    /// been verified ( e.g. the signature of the prekey is valid)
    pub async fn register_agent(
        &self,
        pool: Pool<Postgres>,
        agent: RegisterAgent,
    ) -> Result<(), anyhow::Error> {
        const QUERY: &str = "INSERT INTO agents
        (id, created_at, last_seen_at, identity_public_key, public_prekey, public_prekey_signature)
        VALUES ($1, $2, $3, $4, $5, $6";

        let id = Uuid::new_v4();
        let timestamp = chrono::Utc::now();

        sqlx::query(QUERY)
            .bind(id)
            .bind(timestamp)
            .bind(timestamp)
            .bind(agent.public_identity_key)
            .bind(agent.public_prekey)
            .bind(agent.public_prekey_signature)
            .execute(&pool)
            .await?;

        Ok(())
    }

    ///Returns a list of all agents registered in the server
    pub async fn get_all_agents(&self, db: Pool<Postgres>) -> Result<Vec<Agent>, Error> {
        const QUERY_STR: &str = "SELECT * FROM agents";

        let res = query_as::<_, Agent>(QUERY_STR).fetch_all(&db).await?;

        Ok(res)
    }
}
