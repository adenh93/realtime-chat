use crate::{domain::Connection, errors::CommandError, traits::CommandApply};
use async_trait::async_trait;
use futures::SinkExt;

#[derive(Debug, PartialEq)]
pub struct Help {}

impl Help {}

#[async_trait]
impl CommandApply for Help {
    async fn apply(&self, conn: &mut Connection) -> Result<(), CommandError> {
        // TODO: Generate Command list
        conn.lines
            .send(String::from("Reply from HELP"))
            .await
            .map_err(|e| CommandError::ExecutionError(e.to_string()))?;

        Ok(())
    }
}
