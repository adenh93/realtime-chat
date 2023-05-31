use crate::{domain::Connection, errors::CommandError, frame::Frame, traits::CommandApply};
use async_trait::async_trait;

#[derive(Debug, PartialEq)]
pub struct Me {
    message: String,
}

impl Me {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

#[async_trait]
impl CommandApply for Me {
    async fn apply(&self, conn: &mut Connection) -> Result<(), CommandError> {
        let mut state = conn.state.lock().await;
        let message = format!("{} is {}", conn.peer.username.to_string(), self.message);
        let frame = Frame::ServerMessage(message);

        state.broadcast(conn.peer.addr, frame).await;

        Ok(())
    }
}

impl TryFrom<Vec<&str>> for Me {
    type Error = CommandError;

    fn try_from(args: Vec<&str>) -> Result<Self, Self::Error> {
        if args.len() == 0 {
            return Err(CommandError::MissingArgument("message".into()));
        }

        let message = args.join(" ");

        Ok(Self { message })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use fake::{
        faker::lorem::en::{Sentence, Word},
        Fake,
    };

    #[test]
    fn parses_me_command() {
        let word: &str = &Word().fake::<String>();
        let args = vec![word];

        let command = Me::try_from(args);
        let expected = Me::new(word.into());

        assert_eq!(command, Ok(expected));
    }

    #[test]
    fn parses_whitespace_delimited_sentence() {
        let sentence: &str = &Sentence(0..2).fake::<String>();
        let args = vec![sentence];

        let command = Me::try_from(args);
        let expected = Me::new(sentence.into());

        assert_eq!(command, Ok(expected));
    }

    #[test]
    fn returns_error_if_missing_message_argument() {
        let args = vec![];

        let command = Me::try_from(args);
        let expected = CommandError::MissingArgument("message".into());

        assert_eq!(command, Err(expected));
    }
}
