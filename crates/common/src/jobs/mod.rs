use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum JobError {
    #[error("Identity Key already registered to an agent")]
    IdentityAlreadyTaken,
    #[error("Identity Keys oncelocks have already been initialised")]
    IdentityAlreadyInitialised,
}
