use anyhow::{Error, anyhow};
use common::schemas::{Agent, RegisterAgent};
use serde::Serialize;
use sqlx::{Pool, Postgres, query_as};
use tracing::error;
use uuid::Uuid;

use crate::services;

use super::Repository;

impl Repository {
    ///Creates a row in the agents table for a newly created agent. This assumes that the validity of the record has already
    /// been verified ( e.g. the signature of the prekey is valid)
    pub async fn register_agent(
        &self,
        pool: &Pool<Postgres>,
        agent: services::Agent,
    ) -> Result<(), anyhow::Error> {
        const QUERY: &str = "INSERT INTO agents
        (id, created_at, last_seen_at, identity_public_key, public_prekey, public_prekey_signature)
        VALUES ($1, $2, $3, $4, $5, $6";

        sqlx::query(QUERY)
            .bind(agent.id)
            .bind(agent.created_at)
            .bind(agent.last_seen_at)
            .bind(agent.identity_public_key)
            .bind(agent.public_prekey)
            .bind(agent.public_prekey_signature)
            .execute(pool)
            .await?;

        Ok(())
    }

    ///Returns a list of all agents registered in the server
    pub async fn get_all_agents(&self, db: &Pool<Postgres>) -> Result<Vec<services::Agent>, Error> {
        const QUERY_STR: &str = "SELECT * FROM agents";

        let res = query_as::<_, services::Agent>(QUERY_STR)
            .fetch_all(db)
            .await?;

        Ok(res)
    }

    ///Updates an agent record's last seen at timestamp whenever an agent makes contact with the server
    pub async fn update_agent(
        &self,
        db: &Pool<Postgres>,
        agent: &services::Agent,
    ) -> Result<(), Error> {
        const QUERY: &str = "UPDATE agents
            SET last_seen_at = $1
            WHERE id = $2";

        match sqlx::query(QUERY)
            .bind(agent.last_seen_at)
            .bind(agent.id)
            .execute(db)
            .await
        {
            Err(err) => {
                error!("update_agent: updating agent {}", &err);
                Err(err.into())
            }

            Ok(_) => Ok(()),
        }
    }

    pub async fn find_agent_by_id(
        &self,
        db: &Pool<Postgres>,
        agent_id: Uuid,
    ) -> Result<services::Agent, Error> {
        const QUERY: &str = "SELECT * FROM agents 
            WHERE id = $1";

        match sqlx::query_as::<_, services::Agent>(QUERY)
            .bind(agent_id)
            .fetch_optional(db)
            .await
        {
            Err(err) => {
                error!("find_agent_by_id : find agent {}", &err);
                Err(err.into())
            }

            Ok(None) => Err(anyhow!("No agent found.")),
            Ok(Some(res)) => Ok(res),
        }
    }
}
