use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum JobError {
    #[error("Identity Key already registered to an agent")]
    IdentityAlreadyTaken,
    #[error("Identity Keys oncelocks have already been initialised")]
    IdentityAlreadyInitialised,
    #[error("Failed to send Ureq Request, Error: {}", .0)]
    UreqSendFailure(String),
    #[error("Failed to read Ureq Response, Error: {}", .0)]
    UreqResponseReadFailure(String),
    #[error("Api Error: {}", .0)]
    ApiErr(String),
    #[error("Client Identity public key error: {}", .0)]
    ClientIdentityKeyDecodeFailure(String),
    #[error("Agent not found error: {}",.0)]
    AgentNotFound(String),
    #[error("Agents list error: Agent list empty")]
    NoRegisteredAgents,
}
