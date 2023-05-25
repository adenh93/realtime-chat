use super::Lines;
use futures::{SinkExt, StreamExt};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Username(pub String);

impl Username {
    pub async fn from_frame(lines: &mut Lines) -> Result<Self, String> {
        // TODO: Username + tripcode phrase should be entered via CLI command args
        lines.send("Enter username").await.unwrap();

        match lines.next().await {
            Some(Ok(username)) => Ok(Self(username)),
            _ => Err(String::from("Unable to read username from frame")),
        }
    }
}
