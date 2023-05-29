use crate::{domain::Connection, errors::CommandError, traits::CommandApply, utils::try_pop_arg};
use async_trait::async_trait;
use futures::SinkExt;

pub struct Whisper {
    username: String,
    message: String,
}

impl Whisper {
    fn format_message(&self, prefix: &str, username: &str) -> String {
        format!("{} {}: {}", prefix, username, self.message)
    }
}

#[async_trait]
impl CommandApply for Whisper {
    async fn apply(&self, conn: &mut Connection) -> Result<(), CommandError> {
        let state = conn.state.lock().await;

        // If the sender tries to message their own username, do nothing.
        if self.username == conn.peer.username.0 {
            return Ok(());
        }

        // Locate the connected peer to address the private message to,
        // if possible. Otherwise return an error to the sender.
        let target_peer = state
            .peers
            .iter()
            .find(|peer| peer.0 .0 == self.username)
            .ok_or_else(|| {
                CommandError::ExecutionError(format!("No user with username {}", self.username))
            })?;

        // TODO: Implement System Message frame
        let to_message = self.format_message("To", &self.username);
        let from_message = self.format_message("From", &conn.peer.username.0);

        // Send the message directly to the connected peer
        target_peer.1.tx.send(from_message).await.map_err(|_| {
            CommandError::ExecutionError(format!("Failed to send whisper to {}", self.username))
        })?;

        // Also send a copy to the sender's stream too.
        // TODO: Review if this should quietly fail
        let _ = conn.lines.send(to_message).await;

        Ok(())
    }
}

impl TryFrom<Vec<&str>> for Whisper {
    type Error = CommandError;

    fn try_from(args: Vec<&str>) -> Result<Self, Self::Error> {
        let mut args = args.iter();

        let username = try_pop_arg(&mut args, "username")?;
        let message = try_pop_arg(&mut args, "message")?;

        // Join the rest of the message with space delimiter
        let message = args.fold(message, |a, b| format!("{a} {b}"));

        Ok(Self { username, message })
    }
}
