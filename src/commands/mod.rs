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
            cmd if cmd.is_empty() => Err(CommandError::MissingName)?,
            cmd => Err(CommandError::UnknownCommand(cmd.into()))?,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use fake::faker::internet::en::Username;
    use fake::faker::lorem::en::Sentence;
    use fake::{Fake, Faker};

    #[test]
    fn parses_valid_command() {
        let username: String = Username().fake();
        let message: String = Sentence(0..2).fake();

        let value = &format!("whisper {} {}", &username, &message);
        let command = Command::try_from(value.as_str());
        let expected = Command::Whisper(Whisper::new(username, message));

        assert_eq!(command, Ok(expected));
    }

    #[test]
    fn returns_error_if_command_name_is_empty() {
        let value = String::new();
        let command = Command::try_from(value.as_str());

        assert_eq!(command, Err(CommandError::MissingName));
    }

    #[test]
    fn returns_error_if_command_name_is_whitespace() {
        let value = String::from(" ");
        let command = Command::try_from(value.as_str());

        assert_eq!(command, Err(CommandError::MissingName));
    }

    #[test]
    fn returns_error_if_command_name_is_not_registered() {
        let value = Faker.fake::<String>();
        let command = Command::try_from(value.as_str());

        assert_eq!(command, Err(CommandError::UnknownCommand(value)));
    }
}
