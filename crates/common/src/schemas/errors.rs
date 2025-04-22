use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Record not found")]
    NotFound,
    #[error("n/a error")]
    StandardErr,
}
