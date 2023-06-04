use thiserror::Error;
use super::token::token::Token;

#[derive(Error, Debug)]
pub enum CmdError {
    #[error("Missing path")]
    MissingPath,
    #[error("Missing token: '{0}'")]
    MissingToken(Token),
    #[error("Missing statement")]
    MissingStatement,
    #[error("Missing argument")]
    MissingArg,
    #[error("Missing value")]
    MissingValue,
    #[error("Missing key")]
    MissingKey,
    #[error("Missing subcommand")]
    MissingSubCmd,

    #[error("Incorrect argument to command '{0}'")]
    ParameterError(String),

    #[error("Unexpected token: '{0}'")]
    UnexpectedToken(Token),

    #[error("Unknown error")]
    Unknown,
}

pub type Result<T> = std::result::Result<T, CmdError>;
