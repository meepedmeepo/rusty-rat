mod api_calls;
mod config;
mod parser;

pub use api_calls::*;
use chacha20poly1305::{KeyInit, XChaCha20Poly1305, aead::Aead};
use common::{
    cryptographic_functions::{XCHACHA20_POLY1305_NONCE_SIZE, derive_key_from_shared_secret},
    jobs::JobError,
    schemas::{CreateJob, Job, JobPayload},
};
pub use config::*;
use ed25519_dalek::{Signature, SigningKey, VerifyingKey, ed25519::signature::SignerMut};
pub use parser::*;
use rand::{TryRngCore, rngs::OsRng};
use ureq::{Agent, config::Config};
use uuid::Uuid;
use x25519_dalek::{X25519_BASEPOINT_BYTES, x25519};
use zeroize::Zeroize;

use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
    time::Duration,
};

use base64::Engine;
pub use common;

pub fn init() -> ureq::Agent {
    ureq::Agent::new_with_config(
        Config::builder()
            .https_only(true)
            .timeout_global(Some(Duration::from_secs(10)))
            .build(),
    )
}

pub fn create_client_identity() {
    use base64::engine::general_purpose::STANDARD;

    let (priv_key, pub_key) = common::cryptographic_functions::create_identity();

    let priv_str = STANDARD.encode(priv_key.to_bytes());
    let pub_str = STANDARD.encode(pub_key.as_bytes());

    fs::write(
        "client_identity.secret",
        format!("{}\n{}\n", priv_str, pub_str),
    )
    .unwrap();
}

pub fn encrypt_job(
    api_client: &Agent,
    agent_uuid: Uuid,
    job: JobPayload,
) -> Result<CreateJob, JobError> {
    let agent = fetch_agent_single(agent_uuid, api_client)?;
    let verify = ed25519_dalek::VerifyingKey::from_bytes(&agent.identity_public_key)
        .map_err(|err| JobError::AgentPublicIdentityKeyInvalid(err.to_string()))?;

    let prekey_to_verify = agent.public_prekey.to_vec();

    let signature = Signature::try_from(&agent.public_prekey_signature[0..64])
        .map_err(|err| JobError::PrekeySignatureInvalid(err.to_string()))?;

    if verify.verify_strict(&prekey_to_verify, &signature).is_err() {
        return Err(JobError::SignaturesDontMatch);
    }

    let mut rand_gen = OsRng {};
    //create one time keys for job encryption
    let mut ephemeral_private_key = [0u8; 32];
    rand_gen.try_fill_bytes(&mut ephemeral_private_key).unwrap();
    let ephemeral_public_key = x25519(ephemeral_private_key.clone(), X25519_BASEPOINT_BYTES);

    //create nonce for job encryption
    let mut nonce = [0u8; XCHACHA20_POLY1305_NONCE_SIZE];
    rand_gen.try_fill_bytes(&mut nonce).unwrap();

    //do key exchange between ephemeral keys and agents public prekey to create shared secret
    let mut shared_secret = x25519(ephemeral_private_key, agent.public_prekey);

    let mut key = derive_key_from_shared_secret(&shared_secret, &nonce);

    let job_payload_json = serde_json::to_vec(&job)
        .map_err(|err| JobError::PayloadSerializationFailed(err.to_string()))?;

    let cipher = XChaCha20Poly1305::new(key.as_ref().into());

    let encrypted_job_payload = cipher
        .encrypt(&nonce.into(), job_payload_json.as_ref())
        .map_err(|err| JobError::JobEncryptionFailed(err.to_string()))?;

    shared_secret.zeroize();
    key.zeroize();

    let job_id = Uuid::new_v4();
    let mut buffer_to_sign = job_id.as_bytes().to_vec();
    buffer_to_sign.append(&mut agent_uuid.as_bytes().to_vec());
    buffer_to_sign.append(&mut encrypted_job_payload.clone());
    buffer_to_sign.append(&mut ephemeral_public_key.to_vec());
    buffer_to_sign.append(&mut nonce.to_vec());

    let (mut priv_ident, _) = load_identity();

    let job_signature = priv_ident.sign(&buffer_to_sign);

    Ok(CreateJob {
        id: job_id,
        agent_id: agent_uuid,
        encrypted_job: encrypted_job_payload,
        ephemeral_public_key,
        nonce,
        signature: job_signature.to_bytes().to_vec(),
    })
}

fn load_identity() -> (SigningKey, VerifyingKey) {
    use base64::engine::general_purpose::STANDARD;

    let f = File::open("../client_identity.secret").expect("Couldn't open client identity file!");
    let lines = BufReader::new(f)
        .lines()
        .filter_map(|l| if let Ok(line) = l { Some(line) } else { None })
        .collect::<Vec<String>>();

    let priv_str = lines[0].clone();
    let pub_str = lines[1].clone();

    let priv_key = SigningKey::try_from(&STANDARD.decode(priv_str).unwrap()[0..32]).unwrap();
    let pub_key = VerifyingKey::try_from(&STANDARD.decode(pub_str).unwrap()[0..32]).unwrap();

    (priv_key, pub_key)
}
