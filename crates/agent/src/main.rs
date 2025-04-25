use std::time::Duration;

use common::*;
use ureq::config::Config;

fn main() -> Result<(), anyhow::Error> {
    println!("Hello, world!");

    let api_client = ureq::Agent::new_with_config(
        Config::builder()
            .https_only(true)
            .timeout_global(Some(Duration::from_secs(10)))
            .build(),
    );

    let conf = agent::init(&api_client)?;

    Ok(())
}
