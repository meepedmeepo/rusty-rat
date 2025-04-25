use std::sync::{Arc, Mutex};

use super::super::super::services;
use crate::configuration::{AppConfig, AppState};
use anyhow::anyhow;
use axum::{Json, Router, extract::State, routing::post};
use common::schemas::{AgentRegistered, RegisterAgent, Response};

pub fn create_agent_routes(state: services::Service) {
    let agent_routes: Router<services::Service> = Router::new()
        .route("/", post(post_agent_handler))
        .with_state(state);
}

/// ## Handler: POST /api/agents/
/// Attempts to register a new agent
async fn post_agent_handler(
    State(state): State<services::Service>,
    Json(payload): Json<RegisterAgent>,
) -> (Json<Response<AgentRegistered>>) {
    //state.lock().unwrap().

    let res = state.register_agent(payload).await;

    match res {
        Ok(agent) => axum::Json(Response::ok(agent)),

        Err(err) => axum::Json(Response::err(common::schemas::Error::from_error(err))),
    }
}
