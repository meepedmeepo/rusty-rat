use std::{net::Ipv4Addr, ops::Deref};

use anyhow::{Error, Result};
use axum::{Json, Router, routing::get};
use axum_server::tls_rustls::RustlsConfig;
use env_logger::Env;
use log::debug;
use rat_server::{api::routes::api_router, configuration::DB_CONNECTION_STRING};
use serde_json::{Value, json};
use tokio::net::TcpListener;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    unsafe {
        std::env::set_var("RUST_LOG", "DEBUG");
    }
    env_logger::init();

    let app_state = rat_server::configuration::init()?;

    let db = rat_server::services::connect(DB_CONNECTION_STRING).await?;

    let state = rat_server::services::Service::new(db, app_state.clone());

    debug!("Server Port: {}", app_state.lock().unwrap().port,);

    let app = Router::new()
        .route("/api/test", get(test))
        .nest("/api", api_router(state.clone()))
        .with_state(state);
    //let listener = TcpListener::bind("127.0.0.1:6969").await?;
    let addr = std::net::SocketAddr::from((Ipv4Addr::LOCALHOST, app_state.lock().unwrap().port));

    axum_server::bind_rustls(
        addr,
        RustlsConfig::from_pem_file(
            "src/configuration/r-serv.cert.pem",
            "src/configuration/r-serv.key.pem",
        )
        .await?,
    )
    .serve(app.into_make_service())
    .await?;

    Ok(())
}

pub async fn test() -> Json<Value> {
    Json(json!({"message" : "nuggets"}))
}
