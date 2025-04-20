use core::error;

use common::jobs::JobError;
use config::{AGENT_IDENTITY_PUBLIC, AGENT_IDENTITY_SECRET};
//use anyhow::Error;
pub mod config;

pub fn init() {
    match register_client() {
        Ok(()) => {}

        Err(err) => {
            //error!()
            panic!("{}", err);
        }
    }
}

pub fn register_client() -> Result<(), JobError> {
    let (sign, verify) = common::cryptographic_functions::create_identity();

    AGENT_IDENTITY_SECRET
        .set(sign)
        .map_err(|_err| JobError::IdentityAlreadyInitialised)?;

    AGENT_IDENTITY_PUBLIC
        .set(verify)
        .map_err(|_err| JobError::IdentityAlreadyInitialised)?;
    //todo!()

    Ok(())
}
