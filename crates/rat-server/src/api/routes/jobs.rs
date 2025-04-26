use std::time::Duration;

use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{get, post},
};
use common::schemas::{AgentJob, CreateJob, Error, Job, Response, UpdateJobResult};
use tokio::time::sleep;
use uuid::Uuid;

use crate::services;

pub fn create_job_routes(state: services::Service) -> Router<services::Service> {
    Router::new()
        .route("/", post(create_job))
        .route("/result", get(get_job_result).post(post_job_result))
        .route("/fetchjob", get(get_agent_job))
        .with_state(state)
}

/// ## Handler: POST /api/jobs/
/// Attempts to create a new job record
async fn create_job(
    State(state): State<services::Service>,
    Json(input): Json<CreateJob>,
) -> (StatusCode, Json<Response<Job>>) {
    match state.create_job(&input).await {
        Err(err) => (
            StatusCode::BAD_REQUEST,
            axum::Json(Response::from_anyhow_err(err)),
        ),

        Ok(job) => (StatusCode::OK, axum::Json(Response::ok(job))),
    }
}

/// ## Handler: GET /api/jobs/result
/// returns Job struct if the result has been uploaded to the database
async fn get_job_result(
    State(state): State<services::Service>,
    Json(job_id): Json<Uuid>,
) -> (StatusCode, Json<Response<Option<Job>>>) {
    let sleep_time = Duration::from_secs(1);

    //5 seconds long poll
    for _ in 0..5u64 {
        match state.get_job_result(job_id).await {
            Ok(res) => match res {
                None => sleep(sleep_time).await,

                Some(job) => {
                    let job: Job = job.into();
                    return (StatusCode::OK, axum::Json(Response::ok(Some(job))));
                }
            },
            Err(_) => {
                sleep(sleep_time).await;
            }
        }
    }

    //returns none if no job result is found
    (StatusCode::OK, axum::Json(Response::ok(None)))
}

// ## Handler: POST /api/jobs/result
// Updates job record with output of the job from the agent
async fn post_job_result(
    State(state): State<services::Service>,
    Json(job_result): Json<UpdateJobResult>,
) -> (StatusCode, Json<Response<bool>>) {
    match state.update_job_result(job_result).await {
        Err(err) => (
            StatusCode::BAD_REQUEST,
            axum::Json(Response::err(Error::from_error(err))),
        ),

        Ok(()) => (StatusCode::OK, axum::Json(Response::ok(true))),
    }
}

/// ## Handler: GET /api/jobs/fetchjob
async fn get_agent_job(
    State(state): State<services::Service>,
    Json(agent_id): Json<Uuid>,
) -> (StatusCode, Json<Response<Option<AgentJob>>>) {
    let sleep_time = Duration::from_secs(1);

    for _ in 0..5u64 {
        match state.get_agent_job(agent_id).await {
            Err(err) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    axum::Json(Response::from_anyhow_err(err)),
                );
            }

            Ok(job) => match job {
                None => {
                    tokio::time::sleep(sleep_time).await;
                }

                Some(job) => {
                    let agent_job = AgentJob {
                        id: job.id,
                        encrypted_job: job.encrypted_job,
                        ephemeral_public_key: job
                            .ephemeral_public_key
                            .try_into()
                            .expect("get_agent_job: invalid public key"),
                        nonce: job.nonce.try_into().expect("get_agent_job invalid nonce"),
                        signature: job.signature,
                    };

                    return (StatusCode::OK, axum::Json(Response::ok(Some(agent_job))));
                }
            },
        }
    }
    (StatusCode::OK, axum::Json(Response::ok(None)))
}
