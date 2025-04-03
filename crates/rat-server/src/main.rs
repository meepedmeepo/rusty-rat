use std::net::Ipv4Addr;

use anyhow::{Error, Result};
use axum::{routing::get, Json, Router};
use axum_server::tls_rustls::RustlsConfig;
use env_logger::Env;
use log::debug;
use serde_json::{json, Value};
use tokio::net::TcpListener;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    unsafe {
        std::env::set_var("RUST_LOG", "DEBUG");
    }
    env_logger::init();

    //println!("Hello, world!");
    let settings = rat_server::configuration::init()?;

    debug!("Server Port: {}", settings.get::<i32>("port").unwrap());


    
    let app = Router::new().route("/api/test", get(test));
    //let listener = TcpListener::bind("127.0.0.1:6969").await?;
    let addr = std::net::SocketAddr::from((Ipv4Addr::LOCALHOST, settings.get::<u16>("port")?));

    axum_server::bind_rustls(addr,
        RustlsConfig::from_pem_file("src/configuration/r-serv.cert.pem", "src/configuration/r-serv.key.pem").await?)
            .serve(app.into_make_service())
            .await?;
    
    Ok(())
}

pub async fn test() -> Json<Value>
{
    Json(json!({"message" : "nuggets"}))
}