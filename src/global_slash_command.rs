
use std::fmt::{Debug, Error};
use std::future::Future;
use futures::future::BoxFuture;
use serenity::async_trait;
use serenity::client::Context;
use serenity::model::application::interaction::Interaction;
use serenity::model::prelude::command::CommandOption;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;

#[derive(Clone)]
pub struct GlobalSlashCommandDetails
{
    pub name: String,
    pub description: String,
    pub options: Vec<CommandOption>,
    //pub handler: fn(&ApplicationCommandInteraction, &Context, &Interaction) -> Result<(),String>
    pub handler: for<'a> fn(&'a ApplicationCommandInteraction, &'a Context, &'a Interaction) -> BoxFuture<'a, Result<CommandSuccess,CommandError>>
}



// #[async_trait]
// pub trait CommandHandler: Send + Sync{
//     async fn handle_request(&self, command_interaction: &ApplicationCommandInteraction, context: & Context, interaction: &Interaction) -> Result<(),String>;
// }

pub trait GetSlashCommandDetails {
    fn get_slash_command_details() -> GlobalSlashCommandDetails;
}

// pub trait GetCommandFunc {
//     fn handle_request(command_interaction: &ApplicationCommandInteraction, context: & Context, interaction: &Interaction) -> Result<(),String>;
// }

pub enum CommandError {
    InternalError(String),
    InvalidUserInputError(String),
}

pub enum  CommandSuccess{
    Success,
    SuccessWithReply(String)
}

// impl From<Error> for Result<CommandSuccess,CommandError>{
//     fn from(value: T) -> Self {
//         Err(CommandError::InternalError(value))
//     }
// }

pub trait ToCommandResult<T>{
    fn to_command_result(self) -> Result<T,CommandError>;
}

impl<T,E> ToCommandResult<T> for Result<T,E>
    where E: Debug
{
    fn to_command_result(self: Result<T,E>) -> Result<T, CommandError>
    {
        match self {
            Ok(v) => Ok(v),
            Err(e)=> Err(CommandError::InternalError(format!("{:#?}", e)))
        }
    }
}

// impl From<T> for CommandError
//     where T: std::error::Error
// {
//     fn from(value: T) -> Self {
//         let error_message = format!("{:#?}",value);
//         CommandResult::InternalError(error_message)
//     }
// }



//
//
// #[async_trait]
// pub trait GlobalSlashCommand : GetCommandDetails + CommandHandler + Sync + Send {}

