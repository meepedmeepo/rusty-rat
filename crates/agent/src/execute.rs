use std::{process::Command, thread::sleep, time::Duration};

use crate::config;
use anyhow::{Error, anyhow};
use chacha20poly1305::{
    self, KeyInit, XChaCha20Poly1305,
    aead::{Aead, OsRng, rand_core::RngCore},
};
use common::{
    cryptographic_functions::{
        X25519_KEY_SIZE, XCHACHA20_POLY1305_NONCE_SIZE, derive_key_from_shared_secret,
    },
    schemas::{AgentJob, JobPayload, JobResult, Response, UpdateJobResult},
};
use ed25519_dalek::ed25519::signature::SignerMut;
use uuid::Uuid;
use x25519_dalek::{X25519_BASEPOINT_BYTES, x25519};
use zeroize::Zeroize;

pub fn run(api_client: &ureq::Agent, conf: config::Config) -> ! {
    let sleep_time = Duration::from_secs(1);
    let get_job_route = format!("{}/api/agents/{}/job", config::SERVER_URL, conf.agent_id);
    let post_job_result_route = format!("{}/api/job/result", config::SERVER_URL);
    loop {
        let server_res = match api_client.get(get_job_route.as_str()).call() {
            Ok(res) => res,

            Err(err) => {
                log::debug!("Error fetching job : {}", err);
                sleep(sleep_time);
                continue;
            }
        };

        let api_res: Response<AgentJob> = match server_res.into_body().read_json() {
            Ok(res) => res,

            Err(err) => {
                log::debug!("Error parsing response JSON : {}", err);
                sleep(sleep_time);
                continue;
            }
        };

        log::debug!("API response successfully recieved");

        let encrypted_job = match api_res.data {
            Some(job) => job,

            None => {
                log::debug!("No job found! Trying again in : {:?}", sleep_time);
                sleep(sleep_time);
                continue;
            }
        };

        let (job_id, job) = match decrypt_and_verify(&conf, encrypted_job) {
            Ok(res) => res,

            Err(err) => {
                log::debug!("Error encrypting job: {}", err);
                sleep(sleep_time);
                continue;
            }
        };

        let output = execute(job.command, job.args);
        let job_result = match encrypt_and_sign_job_result(
            &mut conf.clone(),
            job_id,
            output,
            job.result_ephemeral_public_key,
        ) {
            Ok(res) => res,

            Err(err) => {
                log::debug!("Error encrypting job result: {}", err);
                sleep(sleep_time);
                continue;
            }
        };

        match api_client
            .post(post_job_result_route.as_str())
            .send_json(job_result)
        {
            Ok(_) => {}

            Err(err) => {
                log::debug!("Error sending job result, {}", err);
            }
        };
    }
}

fn execute(command: String, args: Vec<String>) -> String {
    let mut ret = String::new();

    let output = match Command::new(command).args(&args).output() {
        Ok(output) => output,
        Err(err) => {
            log::debug!("Error executing command: {}", err);
            return ret;
        }
    };

    ret = match String::from_utf8(output.stdout) {
        Ok(stdout) => stdout,
        Err(err) => {
            log::debug!("Error converting command's output to String: {}", err);
            return ret;
        }
    };

    return ret;
}

///Attempts to decrypt job and if successful returns a JobPayload struct and the job id
fn decrypt_and_verify(conf: &config::Config, job: AgentJob) -> Result<(Uuid, JobPayload), Error> {
    let mut buffer_to_verify = job.id.as_bytes().to_vec();
    buffer_to_verify.append(&mut conf.agent_id.as_bytes().to_vec());
    buffer_to_verify.append(&mut job.encrypted_job.clone());
    buffer_to_verify.append(&mut job.ephemeral_public_key.to_vec());
    buffer_to_verify.append(&mut job.nonce.to_vec());

    let signature = ed25519_dalek::Signature::try_from(&job.signature[0..64])?;

    if conf
        .client_identity_public_key
        .verify_strict(&buffer_to_verify, &signature)
        .is_err()
    {
        return Err(anyhow!("Agent's prekey signature is invalid".to_string()));
    }

    let mut shared_secret = x25519(conf.private_prekey, job.ephemeral_public_key);

    let mut key = derive_key_from_shared_secret(&shared_secret, &job.nonce);

    let cipher = XChaCha20Poly1305::new(key.as_ref().into());
    let decrypted_job_bytes = cipher
        .decrypt(&job.nonce.into(), job.encrypted_job.as_ref())
        .map_err(|err| anyhow!("Couldn't decrypt job error {}", err))?;

    shared_secret.zeroize();
    key.zeroize();

    let job_payload: JobPayload = serde_json::from_slice(&decrypted_job_bytes)?;

    Ok((job.id, job_payload))
}

fn encrypt_and_sign_job_result(
    conf: &mut config::Config,
    job_id: Uuid,
    output: String,
    job_result_ephemeral_public_key: [u8; X25519_KEY_SIZE],
) -> Result<UpdateJobResult, Error> {
    let mut rand_generator = OsRng {};

    //create ephemeral keys
    let mut ephemeral_private_key = [0u8; X25519_KEY_SIZE];
    rand_generator.fill_bytes(&mut ephemeral_private_key);
    let ephemeral_public_key = x25519(ephemeral_private_key.clone(), X25519_BASEPOINT_BYTES);

    //key exchange
    let mut shared_secret = x25519(ephemeral_private_key, job_result_ephemeral_public_key);

    //create nonce
    let mut nonce = [0u8; XCHACHA20_POLY1305_NONCE_SIZE];
    rand_generator.fill_bytes(&mut nonce);

    //derive encryption key
    let mut key = derive_key_from_shared_secret(&shared_secret, &nonce);

    //serialize job result into JSON
    let job_result_payload = JobResult { result: output };
    let job_result_payload_json = serde_json::to_vec(&job_result_payload)?;

    //encrypt job
    let cipher = XChaCha20Poly1305::new(key.as_ref().into());
    let encrypted_job_result = cipher
        .encrypt(&nonce.into(), job_result_payload_json.as_ref())
        .map_err(|err| anyhow!("Job result encryption error : {}", err))?;

    shared_secret.zeroize();
    key.zeroize();

    let mut buffer_to_sign = job_id.as_bytes().to_vec();
    buffer_to_sign.append(&mut conf.agent_id.as_bytes().to_vec());
    buffer_to_sign.append(&mut encrypted_job_result.clone());
    buffer_to_sign.append(&mut ephemeral_public_key.to_vec());
    buffer_to_sign.append(&mut nonce.to_vec());

    let signature = conf.identity_secret_key.sign(&buffer_to_sign);

    Ok(UpdateJobResult {
        job_id,
        encrypted_job_result,
        ephemeral_public_key,
        nonce,
        signature: signature.to_bytes().to_vec(),
    })
}
