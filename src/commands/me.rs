use crate::{domain::Connection, errors::CommandError, traits::CommandApply};
use async_trait::async_trait;

#[derive(Debug, PartialEq)]
pub struct Me {
    message: String,
}

#[async_trait]
impl CommandApply for Me {
    async fn apply(&self, conn: &mut Connection) -> Result<(), CommandError> {
        let mut state = conn.state.lock().await;
        let message = format!("{} is {}", conn.peer.username.0, self.message);

        state.broadcast(conn.peer.addr, &message).await;

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
