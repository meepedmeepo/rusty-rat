use anyhow::Error;
use log::error;
use sqlx::{self, Pool, Postgres, postgres::PgPoolOptions};
use std::time::Duration;

pub async fn connect(database_url: &str) -> Result<Pool<Postgres>, Error> {
    PgPoolOptions::new()
        .max_connections(20)
        .max_lifetime(Duration::from_secs(30 * 60))
        .connect(database_url)
        .await
        .map_err(|err| {
            error!("db: Connecting to DB: {}", err);
            err.into()
        })
}

pub async fn migrate(db: &Pool<Postgres>) -> Result<(), Error> {
    match sqlx::migrate!("src/migrations").run(db).await {
        Ok(_) => {}

        Err(err) => {
            error!("db::migrate: migrating: {}", err);
            return Err(err.into());
        }
    };

    Ok(())
}
