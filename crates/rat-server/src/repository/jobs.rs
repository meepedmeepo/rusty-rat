use anyhow::{Error, anyhow};
use common::{jobs::JobError, schemas::Job};
use sqlx::{Pool, Postgres};
use tracing::error;
use uuid::Uuid;

use super::Repository;

impl Repository {
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
}
