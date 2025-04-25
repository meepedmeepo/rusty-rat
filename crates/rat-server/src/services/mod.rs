mod agents;
mod entities;
mod jobs;
mod postgres_db;

use std::sync::{Arc, Mutex};

pub use agents::*;
pub use entities::*;
pub use jobs::*;
pub use postgres_db::*;
use sqlx::{Pool, Postgres};

use crate::{configuration::AppConfig, repository::Repository};

/// TODO! Consider adding max job size and max job result size and validation for them in an attempt to stop
/// a badly created job or result causing havoc on the server
#[derive(Debug, Clone)]
pub struct Service {
    repo: Repository,
    db: Pool<Postgres>,
    state: Arc<Mutex<AppConfig>>,
}

impl Service {
    pub fn new(db: Pool<Postgres>, state: Arc<Mutex<AppConfig>>) -> Self {
        let repo = Repository {};

        Self { repo, db, state }
    }
}
