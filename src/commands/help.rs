use crate::{domain::Connection, errors::CommandError, frame::Frame, traits::CommandApply};
use async_trait::async_trait;
use futures::SinkExt;

#[derive(Debug, PartialEq)]
pub struct Help {}

impl Help {}

#[async_trait]
impl CommandApply for Help {
    async fn apply(&self, conn: &mut Connection) -> Result<(), CommandError> {
        // TODO: Generate Command list
        let frame = Frame::ServerMessage(String::from("Reply from HELP"));

        conn.messages
            .send(frame)
            .await
            .map_err(|e| CommandError::ExecutionError(e.to_string()))?;

        Ok(())
    }
}
