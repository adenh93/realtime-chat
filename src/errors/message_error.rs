use super::CommandError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MessageError {
    #[error("Failed to parse message")]
    ParseFailure,
    #[error(transparent)]
    CommandFailure(#[from] CommandError),
}
