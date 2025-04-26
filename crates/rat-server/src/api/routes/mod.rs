use agents::create_agent_routes;
use axum::Router;
use jobs::create_job_routes;

use crate::services;

mod agents;
mod jobs;

pub fn api_router(state: services::Service) -> Router<services::Service> {
    Router::new()
        .nest("/agents", create_agent_routes(state.clone()))
        .nest("jobs", create_job_routes(state.clone()))
        .with_state(state.clone())
}
