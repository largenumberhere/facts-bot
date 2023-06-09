use std::fmt::Debug;

pub enum CommandError {
    InternalError(String),
    InvalidUserInputError(String),
}

pub enum CommandSuccess {
    Success,
    SuccessWithReply(String),
}

pub trait ToCommandResult<T> {
    fn to_command_result(self) -> Result<T, CommandError>;
}

impl<T, E> ToCommandResult<T> for Result<T, E>
where
    E: Debug,
{
    fn to_command_result(self: Result<T, E>) -> Result<T, CommandError> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => Err(CommandError::InternalError(format!("{:#?}", e))),
        }
    }
}

pub trait ToCommandResultWith<T> {
    fn to_command_result_with(self, message: &str) -> Result<T, CommandError>;
}

impl<T, E> ToCommandResultWith<T> for Result<T, E>
where
    E: Debug,
{
    fn to_command_result_with(self, message: &str) -> Result<T, CommandError> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => Err(CommandError::InternalError(format!(
                "Error: {}. Details: {:#?}",
                message, e
            ))),
        }
    }
}
