mod agents;
mod jobs;
pub use agents::*;
pub use jobs::*;

use anyhow::Error;
use common::schemas::Agent;
use sqlx::{Executor, Pool, Postgres, query, query_as};

#[derive(Debug)]
pub struct Repository {}

impl Repository {}
