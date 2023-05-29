use crate::errors::CommandError;
use std::slice::Iter;

/// Attempts to pop an argument from the provided iterator. Returns a
/// `CommandError::MissingArgument` error variant if not found.
pub fn try_pop_arg(args: &mut Iter<&str>, name: &str) -> Result<String, CommandError> {
    let arg = args
        .next()
        .ok_or_else(|| CommandError::MissingArgument(name.into()))?
        .to_string();

    Ok(arg)
}
