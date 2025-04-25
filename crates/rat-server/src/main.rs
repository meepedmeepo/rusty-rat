use std::{net::Ipv4Addr, ops::Deref};

use anyhow::{Error, Result};
use axum::{Json, Router, routing::get};
use axum_server::tls_rustls::RustlsConfig;
use env_logger::Env;
use log::debug;
use rat_server::configuration::APPSTATE;
use serde_json::{Value, json};
use tokio::net::TcpListener;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    unsafe {
        std::env::set_var("RUST_LOG", "DEBUG");
    }
    env_logger::init();

    //println!("Hello, world!");
    rat_server::configuration::init()?;

    debug!("Server Port: {}", APPSTATE.lock().unwrap().config.port,);

    let app = Router::new().route("/api/test", get(test));
    //let listener = TcpListener::bind("127.0.0.1:6969").await?;
    let addr =
        std::net::SocketAddr::from((Ipv4Addr::LOCALHOST, APPSTATE.lock().unwrap().config.port));

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
