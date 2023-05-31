use crate::errors::UsernameError;

use super::{Messages, Tripcode};
use futures::StreamExt;
use std::fmt::Display;

/// A Username is made up of two parts, the nickname portion, and
/// the tripcode portion.
///
/// When a Username is converted to a string via the Display trait,
/// it will have the format `<nickname>!<tripcode>`, for example:
/// some_user!uQ8unuo3Mk
///
/// A Username should not be constructed directly, but rather via
/// the `from_frame` method.
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Username {
    nickname: String,
    tripcode: String,
}

impl Username {
    pub async fn from_frame(messages: &mut Messages) -> Result<Self, UsernameError> {
        // Pull the message off of the stream. The message will
        // have the format nickname,password e.g. some_user,password123
        let message = match messages.next().await {
            Some(Ok(frame)) => frame.message(),
            Some(Err(err)) => Err(UsernameError::ReadFailure(err))?,
            _ => Err(UsernameError::NoData)?,
        };

        // Extract nickname and password from comma-separated pair
        let (nickname, password) = message
            .split_once(',')
            .ok_or_else(|| UsernameError::ParseFailure)?;

        // Generate tripcode from the password provided by the client
        let tripcode = Tripcode::try_from(password.to_owned())
            .map_err(|err| UsernameError::GenTripcodeFailure(err))?;

        Ok(Self {
            nickname: nickname.to_owned(),
            tripcode: tripcode.to_string(),
        })
    }
}

impl Display for Username {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}!{}", self.nickname, self.tripcode)
    }
}
