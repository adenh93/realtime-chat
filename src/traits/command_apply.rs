use crate::{domain::Connection, errors::CommandError};
use async_trait::async_trait;

#[async_trait]
pub trait CommandApply {
    /// Allows a command to be executed.
    //
    // TODO: When async fns in Traits are introduced to Stable Rust, the async_trait
    // macro can be removed.
    async fn apply(&self, conn: &mut Connection) -> Result<(), CommandError>;
}
