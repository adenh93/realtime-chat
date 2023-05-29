use crate::{commands::Command, errors::MessageError};

#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::commands::Help;
    use crate::errors::CommandError;
    use fake::faker::lorem::en::Word;
    use fake::Fake;

    #[test]
    fn parses_command_if_prefixed_with_forward_slash() {
        let value = String::from("/help");
        let message = Message::try_from(value);

        assert_eq!(message, Ok(Message::Cmd(Command::Help(Help {}))));
    }

    #[test]
    fn returns_error_if_fails_to_parse_command() {
        let value = String::from("/");
        let message = Message::try_from(value);

        assert_eq!(
            message,
            Err(MessageError::CommandFailure(CommandError::MissingName))
        )
    }

    #[test]
    fn parses_raw_message() {
        let value: String = Word().fake();
        let message = Message::try_from(value.clone());

        assert_eq!(message, Ok(Message::Raw(value)))
    }

    #[test]
    fn returns_error_if_trying_to_parse_empty_string() {
        let value = String::new();
        let message = Message::try_from(value.clone());

        assert_eq!(message, Err(MessageError::ParseFailure));
    }
}
