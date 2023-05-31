use thiserror::Error;

#[derive(Error, Debug)]
pub enum UsernameError {
    #[error("Failed to read username/password pair from frame: {0}")]
    ReadFailure(#[from] std::io::Error),
    #[error("No username/password read from frame.")]
    NoData,
    #[error("Failed to parse username.")]
    ParseFailure,
    #[error("Failed to generate tripcode: {0}")]
    GenTripcodeFailure(argon2::password_hash::Error),
}
