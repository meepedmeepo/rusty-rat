use common::{
    jobs::JobError,
    schemas::{Agent, AgentsList, CreateJob, Job, Response},
};
use uuid::Uuid;

pub fn fetch_agent_single(uuid: Uuid, api_client: &ureq::Agent) -> Result<Agent, JobError> {
    let api_fetch_agent_route = format!("{}/api/agents/{}", super::SERVER_URL, uuid);
    let res: Response<Agent> = api_client
        .get(api_fetch_agent_route)
        .call()
        .map_err(|err| JobError::UreqSendFailure(err.to_string()))?
        .into_body()
        .read_json()
        .map_err(|err| JobError::UreqResponseReadFailure(err.to_string()))?;

    if let Some(err) = res.error {
        return Err(JobError::ApiErr(err.message));
    }

    match res.data {
        None => Err(JobError::AgentNotFound(uuid.to_string())),

        Some(agent) => Ok(agent),
    }
}

pub fn fetch_all_agents(api_client: &ureq::Agent) -> Result<AgentsList, JobError> {
    let api_fetch_all_agents_route = format!("{}/api/agents", super::SERVER_URL);

    let res: Response<AgentsList> = api_client
        .get(api_fetch_all_agents_route)
        .call()
        .map_err(|err| JobError::UreqSendFailure(err.to_string()))?
        .into_body()
        .read_json()
        .map_err(|err| JobError::UreqResponseReadFailure(err.to_string()))?;

    if let Some(err) = res.error {
        return Err(JobError::ApiErr(err.message));
    }

    match res.data {
        None => Err(JobError::ApiErr(
            "API error: server didn't return AgentList".to_owned(),
        )),

        Some(agent_list) => {
            if agent_list.agents.is_empty() {
                Err(JobError::NoRegisteredAgents)
            } else {
                Ok(agent_list)
            }
        }
    }
}

pub fn post_new_job(api_client: &ureq::Agent, job: CreateJob) -> Result<Job, JobError> {
    let api_post_create_job_route = format!("{}/api/jobs", super::SERVER_URL);

    let res: Response<Job> = api_client
        .post(api_post_create_job_route)
        .send_json(job)
        .map_err(|err| JobError::UreqSendFailure(err.to_string()))?
        .into_body()
        .read_json()
        .map_err(|err| JobError::UreqResponseReadFailure(err.to_string()))?;

    if let Some(err) = res.error {
        return Err(JobError::ApiErr(err.message));
    }

    match res.data {
        None => Err(JobError::ApiErr(
            "API Error: Couldn't create new job".to_owned(),
        )),

        Some(job) => Ok(job),
    }
}
