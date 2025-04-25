use anyhow::Error;
use base64::Engine;
use common::{
    jobs::JobError,
    schemas::{RegisterAgent, Response},
};
use config::{CLIENT_IDENTITY_PUBLIC_KEY, Config};
use core::error;

//use anyhow::Error;
pub mod config;

pub fn init(api_client: &ureq::Agent) -> Result<config::Config, Error> {
    let conf = register_client(api_client)?;

    Ok(conf)
}

pub fn register_client(api_client: &ureq::Agent) -> Result<config::Config, JobError> {
    let (signing_key, verifying_key) = common::cryptographic_functions::create_identity();

    let agent_reg_api_route = format!("{}/api/agents", config::SERVER_URL);
    let (identity_secret_key, identity_public_key) =
        common::cryptographic_functions::create_identity();

    let (prekey_pair, prekey_signature) =
        common::cryptographic_functions::create_and_sign_prekey(&signing_key);

    let register_agent = RegisterAgent {
        public_identity_key: verifying_key.to_bytes(),
        public_prekey: prekey_pair.public_prekey.clone(),
        public_prekey_signature: prekey_signature.to_bytes().to_vec(),
    };

    //sends request to api endpoint on the server to attempt to register the new agent and either returns a UUID of the newly
    //registered agent, or returns an error
    let response: Response<common::schemas::AgentRegistered> = api_client
        .post(agent_reg_api_route.as_str())
        .send_json(register_agent)
        .map_err(|err| JobError::UreqSendFailure(err.to_string()))?
        .into_body()
        .read_json()
        .map_err(|err| JobError::UreqResponseReadFailure(err.to_string()))?;

    if let Some(err) = response.error {
        return Err(JobError::ApiErr(err.message));
    }

    let client_public_key_bytes = base64::engine::general_purpose::STANDARD
        .decode(CLIENT_IDENTITY_PUBLIC_KEY)
        .map_err(|err| JobError::ClientIdentityKeyDecodeFailure(err.to_string()))?;

    let client_identity_public_key = ed25519_dalek::VerifyingKey::from_bytes(
        &client_public_key_bytes.try_into().unwrap_or_default(),
    )
    .map_err(|err| JobError::ClientIdentityKeyDecodeFailure(err.to_string()))?;

    let conf = Config {
        agent_id: response.data.unwrap().id,
        identity_public_key: identity_public_key,
        identity_secret_key: identity_secret_key,
        public_prekey: prekey_pair.public_prekey,
        private_prekey: prekey_pair.private_prekey,
        client_identity_public_key,
    };

    Ok(conf)
}
