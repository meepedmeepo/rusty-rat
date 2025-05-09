use clap::Parser;
use client::Cli;
use common::{
    cryptographic_functions::XCHACHA20_POLY1305_KEY_SIZE,
    schemas::{Agent, CreateJobPayload, JobPayload, Response},
};
use rand::{TryRngCore, rngs::OsRng};
use uuid::Uuid;
use x25519_dalek::{X25519_BASEPOINT_BYTES, x25519};

fn main() {
    //client::create_client_identity();

    let args = Cli::parse();

    let api_client = client::init();

    match args.command {
        client::Commands::Agents(agents) => {
            let cmd = agents.command;
            match cmd {
                client::AgentCommands::Fetch { uuid } => match Uuid::parse_str(&uuid) {
                    Err(err) => {
                        panic!("Error: couldn't parse uuid: {:?}", err);
                    }
                    Ok(uuid) => {
                        //client::fetch_agent_single(uuid, &api_client).
                    }
                },

                client::AgentCommands::ps => {}
            }
        }

        client::Commands::Jobs(jobs) => {
            let cmd = jobs.command;

            match cmd {
                client::JobCommands::create { id, command, args } => match Uuid::parse_str(&id) {
                    Err(err) => panic!("Error: couldn't parse uuid: {:?}", err),

                    Ok(uuid) => {
                        let mut rand_gen = OsRng {};
                        //split args into vector of args
                        let args = args.split_whitespace().map(|s| s.to_string()).collect();

                        //create keys for job result encryption
                        let mut result_ephemeral_private_key = [0u8; XCHACHA20_POLY1305_KEY_SIZE];

                        rand_gen
                            .try_fill_bytes(&mut result_ephemeral_private_key)
                            .unwrap();

                        let result_ephemeral_public_key =
                            x25519(result_ephemeral_private_key.clone(), X25519_BASEPOINT_BYTES);

                        //create and encrypt payload to be executed on agent
                        let plaintext_payload = JobPayload {
                            command,
                            args,
                            result_ephemeral_public_key,
                        };

                        let job_payload = client::encrypt_job(&api_client, uuid, plaintext_payload)
                            .expect("Job encryption failed");

                        //
                        client::post_new_job(&api_client, job_payload).unwrap();
                    }
                },
            }
        }
    }
}
