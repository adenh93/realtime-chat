mod help;
mod me;
mod whisper;

pub use help::*;
pub use me::*;
pub use whisper::*;

use crate::{domain::Connection, errors::CommandError, traits::CommandApply};

#[derive(Debug, PartialEq)]
pub enum Command {
    Help(Help),
    Whisper(Whisper),
    Me(Me),
}

impl Command {
    pub async fn apply(&self, conn: &mut Connection) -> Result<(), CommandError> {
        let fut = match self {
            Command::Help(cmd) => cmd.apply(conn),
            Command::Whisper(cmd) => cmd.apply(conn),
            Command::Me(cmd) => cmd.apply(conn),
        };

        fut.await?;

        Ok(())
    }
}

impl TryFrom<&str> for Command {
    type Error = CommandError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut parts = value.split(" ");
        let name = parts.next().ok_or_else(|| CommandError::MissingName)?;
        let args: Vec<&str> = parts.collect();

        Ok(match name {
            "help" => Self::Help(Help {}),
            "whisper" => Self::Whisper(Whisper::try_from(args)?),
            "me" => Self::Me(Me::try_from(args)?),
            arg if arg.is_empty() => Err(CommandError::MissingName)?,
            arg => Err(CommandError::UnknownCommand(arg.into()))?,
        })
    }
}
