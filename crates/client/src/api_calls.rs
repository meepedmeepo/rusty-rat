use common::{
    jobs::JobError,
    schemas::{Agent, Response},
};
use uuid::Uuid;

pub fn fetch_agent_single(uuid: Uuid, api_client: &ureq::Agent) -> Result<Agent, JobError> {
    let api_fetch_agent_route = format!("{}/api/agents/{}", super::SERVER_URL, uuid);
    let agent: Response<Agent> = api_client
        .get(api_fetch_agent_route)
        .call()
        .map_err(|err| common::jobs::JobError::UreqSendFailure(err.to_string()))?
        .into_body()
        .read_json()
        .map_err(|err| common::jobs::JobError::UreqResponseReadFailure(err.to_string()))?;

    Ok(agent.data.unwrap())
}
