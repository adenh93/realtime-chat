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

#[cfg(test)]
mod test {
    use super::*;
    use fake::{faker::lorem::en::Word, Fake};

    #[test]
    fn successfully_pops_arg() {
        let name = Word().fake();
        let args = vec![name];
        let mut iter = args.iter();

        let result = try_pop_arg(&mut iter, name);

        assert_eq!(result, Ok(name.into()));
    }

    #[test]
    fn returns_error_if_no_arg() {
        let args = vec![];
        let mut iter = args.iter();

        let name = Word().fake();
        let result = try_pop_arg(&mut iter, name);

        assert_eq!(result, Err(CommandError::MissingArgument(name.into())))
    }
}
