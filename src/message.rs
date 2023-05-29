use crate::{commands::Command, errors::MessageError};

pub enum Message {
    Raw(String),
    Cmd(Command),
}

impl TryFrom<String> for Message {
    type Error = MessageError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(match value.chars().next() {
            Some('/') => Self::Cmd(
                Command::try_from(&value[1..]).map_err(|err| MessageError::CommandFailure(err))?,
            ),
            Some(_) => Self::Raw(value),
            _ => Err(MessageError::ParseFailure)?,
        })
    }
}
