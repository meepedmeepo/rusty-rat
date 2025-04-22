use anyhow::Error;
use chrono::Utc;
use common::schemas::{DatabaseError, UpdateJobResult};
use uuid::Uuid;

use super::{Job, Service};

impl Service {
    pub async fn get_job_result(&self, job_id: Uuid) -> Result<Option<Job>, Error> {
        let job = self.repo.find_job_by_id(&self.db, job_id).await?;

        match &job.encrypted_result {
            Some(_) => {
                self.repo.delete_job(&self.db, job_id).await?;
                Ok(Some(super::Job::from(job.into())))
            }

            None => Ok(None),
        }
    }

    pub async fn get_agent_job(&self, agent_id: Uuid) -> Result<Option<Job>, Error> {
        let mut agent = self.repo.find_agent_by_id(&self.db, agent_id).await?;

        agent.last_seen_at = Utc::now();
        //ignores the result as error not really that important
        let _ = self.repo.update_agent(&self.db, &agent).await;

        match self.repo.find_job_for_agent(&self.db, agent_id).await {
            Ok(job) => Ok(Some(job)),
            Err(err) => {
                if let DatabaseError::NotFound =
                    err.downcast_ref().unwrap_or(&DatabaseError::StandardErr)
                //.unwrap_or_else(|| DatabaseError::StandardErr)
                {
                    Ok(None)
                } else {
                    Err(err)
                }
            }
        }
    }

    pub async fn update_job_result(&self, input : UpdateJobResult) -> Result<(), Error> {
        let mut job = self.repo.find_job_by_id(&self.db, input.job_id).await?;
        let agent = self.repo.find_agent_by_id(&self.db, job.agent_id).await?:

        let mut job_result_buffer = input.job_id.as_bytes().to_vec();
        job_result_buffer.append(&mut agent.id.as_bytes().to_vec());
        job_result_buffer.append(&mut input.encrypted_job_result.clone() );
        job_result_buffer.append(&mut input.ephemeral_public_key.to_vec());
        job_result_buffer.append(&mut input.nonce.to_vec());
        todo!()
    }
}
