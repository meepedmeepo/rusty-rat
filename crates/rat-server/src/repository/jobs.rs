use anyhow::{Error, anyhow};
use common::{jobs::JobError, schemas::Job};
use sqlx::{Pool, Postgres};
use tracing::error;
use uuid::Uuid;

use super::Repository;

impl Repository {
    ///Creates a new job for an agent
    pub async fn create_job(&self, db: &Pool<Postgres>, job: &Job) -> Result<(), Error> {
        const QUERY: &str = "INSERT INTO jobs
            (id, encrypted_job, ephemeral_public_key, nonce, signature, agent_id
            VALUES ($1, $2, $3, $4, $5, $6";

        match sqlx::query(QUERY)
            .bind(job.id)
            .bind(&job.encrypted_job)
            .bind(&job.ephemeral_public_key)
            .bind(&job.nonce)
            .bind(&job.signature)
            .bind(job.agent_id)
            .execute(db)
            .await
        {
            Err(err) => {
                error!("create_job : Inserting job {}", &err);
                Err(err.into())
            }

            Ok(_) => Ok(()),
        }
    }

    ///Gets a list of all jobs
    pub async fn get_all_jobs(&self, db: Pool<Postgres>) -> Result<Vec<Job>, anyhow::Error> {
        const QUERY: &str = "SELECT * FROM jobs ORDER BY created_at";

        let res = sqlx::query_as::<_, Job>(QUERY).fetch_all(&db).await?;

        Ok(res)
    }

    ///Returns job object if a job with a match uuid is found, else returns an anyhow error
    /// Will log an error if it was a connection error that causes lack of job to be found
    pub async fn find_job_by_id(&self, db: &Pool<Postgres>, job_id: Uuid) -> Result<Job, Error> {
        const QUERY: &str = "SELECT * FROM jobs WHERE id = $1";

        match sqlx::query_as::<_, Job>(QUERY)
            .bind(job_id)
            .fetch_optional(db)
            .await
        {
            Err(err) => {
                error!("find_job_by_id: finding job: {}", &err);
                Err(err.into())
            }

            Ok(None) => Err(anyhow!("Job not found")),

            Ok(Some(res)) => Ok(res),
        }
    }

    ///Returns a job if there are any non-completed jobs available for the agent.
    pub async fn find_job_for_agent(
        &self,
        db: &Pool<Postgres>,
        agent_id: Uuid,
    ) -> Result<Job, Error> {
        const QUERY: &str = "SELECT * FROM jobs
            WHERE agent_id = $1 AND encrypted_result IS NULL
            LIMIT 1";

        match sqlx::query_as::<_, Job>(QUERY)
            .bind(agent_id)
            .fetch_optional(db)
            .await
        {
            Err(err) => {
                error!("find_job_for_agent: finding job {}", &err);
                Err(err.into())
            }

            Ok(None) => Err(anyhow!("Job not found")),
            Ok(Some(res)) => Ok(res),
        }
    }

    ///Adds the result of a job after the agent has completed it.
    pub async fn update_job(&self, db: &Pool<Postgres>, job: &Job) -> Result<(), Error> {
        const QUERY: &str = "UPDATE jobs
            SET encrypted_result = $1, result_ephemeral_public_key = $2, 
                result_nonce = $3, result_signature = $4
            WHERE id = $5 ";

        match sqlx::query(QUERY)
            .bind(&job.encrypted_result)
            .bind(&job.result_ephemeral_public_key)
            .bind(&job.result_nonce)
            .bind(&job.result_signature)
            .bind(&job.id)
            .execute(db)
            .await
        {
            Err(err) => {
                error!("update_job : updating job {}", &err);
                Err(err.into())
            }

            Ok(_) => Ok(()),
        }
    }

    ///Deletes specified job record
    pub async fn delete_job(&self, db: &Pool<Postgres>, job_id: Uuid) -> Result<(), Error> {
        const QUERY: &str = "DELETE FROM jobs WHERE id = $1";

        sqlx::query(QUERY).bind(job_id).execute(db).await?;

        Ok(())
    }
}
