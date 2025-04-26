use std::sync::{Arc, Mutex};

use super::super::super::services;
use crate::configuration::{AppConfig, AppState};
use anyhow::anyhow;
use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{get, post},
};
use common::schemas::{Agent, AgentRegistered, AgentsList, Error, RegisterAgent, Response};
use uuid::Uuid;

pub fn create_agent_routes(state: services::Service) -> Router<services::Service> {
    Router::new()
        .route("/", post(post_agent_handler).get(get_agents))
        .route("/single", get(get_agent))
        .with_state(state)
}

/// ## Handler: POST /api/agents/
/// Attempts to register a new agent
async fn post_agent_handler(
    State(state): State<services::Service>,
    Json(payload): Json<RegisterAgent>,
) -> (StatusCode, Json<Response<AgentRegistered>>) {
    let res = state.register_agent(payload).await;

    match res {
        Ok(agent) => (StatusCode::OK, axum::Json(Response::ok(agent))),

        Err(err) => (
            StatusCode::BAD_REQUEST,
            axum::Json(Response::err(common::schemas::Error::from_error(err))),
        ),
    }
}

/// ## Handler: GET /api/agents/
/// returns a list of currently registered agents
async fn get_agents(
    State(state): State<services::Service>,
) -> (StatusCode, Json<Response<AgentsList>>) {
    let agents = state.list_agents().await;
    if agents.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            axum::Json(Response::err(common::schemas::Error::from_error(
                agents.err().unwrap(),
            ))),
        );
    }
    let agents = agents.unwrap().into_iter().map(Into::into).collect();

    let res = AgentsList { agents };

    (StatusCode::OK, axum::Json(Response::ok(res)))
}

/// ## Handler: GET /api/agents/single
/// attempts to get and return an agent with a given UUID
async fn get_agent(
    State(state): State<services::Service>,
    Json(agent_id): Json<Uuid>,
) -> (StatusCode, Json<Response<Agent>>) {
    let agent = state.find_agent(agent_id).await;

    match agent {
        Err(err) => (
            StatusCode::NO_CONTENT,
            axum::Json(Response::err(Error::from_error(err))),
        ),

        Ok(agent) => (StatusCode::OK, axum::Json(Response::ok(agent.into()))),
    }
}
