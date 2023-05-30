use super::Messages;
use crate::frame::Frame;
use futures::{SinkExt, StreamExt};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Username(pub String);

impl Username {
    pub async fn from_frame(messages: &mut Messages) -> Result<Self, String> {
        // TODO: Username + tripcode phrase should be entered via CLI command args
        messages
            .send(Frame::ServerMessage("Enter username".into()))
            .await
            .unwrap();

        match messages.next().await {
            Some(Ok(username)) => Ok(Self(username.message())),
            Some(err) => Err(format!("Unable to read username from frame: {:?}", err)),
            _ => Err(String::from("Unable to read username from frame")),
        }
    }
}
