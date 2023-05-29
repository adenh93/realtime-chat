use super::CommandError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum MessageError {
    #[error("Failed to parse message")]
    ParseFailure,
    #[error(transparent)]
    CommandFailure(#[from] CommandError),
}
