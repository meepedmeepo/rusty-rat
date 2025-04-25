use axum::{Json, Router, extract::State, http::StatusCode, routing::post};
use common::schemas::{CreateJob, Job, Response};

use crate::services;

pub fn create_job_routes(state: services::Service) {
    let agent_routes: Router<services::Service> =
        Router::new().route("/", post(create_job)).with_state(state);
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
