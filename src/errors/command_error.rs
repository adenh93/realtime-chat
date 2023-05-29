use thiserror::Error;

const HELP_MSG: &str = "See /help for a list of all commands.";

#[derive(Error, Debug)]
pub enum CommandError {
    #[error("Too many arguments provided to command.")]
    TooManyArguments,
    #[error("No command entered. {HELP_MSG}")]
    MissingName,
    #[error("Missing command argument: {0}.")]
    MissingArgument(String),
    #[error("Failed to execute command: {0}.")]
    ExecutionError(String),
    #[error("Unknown command: {0}. {HELP_MSG}")]
    UnknownCommand(String),
}
