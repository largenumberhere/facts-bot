use std::fmt::{Debug, Error};
use std::future::Future;
use futures::future::BoxFuture;
use serenity::async_trait;
use serenity::client::Context;
use serenity::model::application::interaction::Interaction;
use serenity::model::prelude::command::CommandOption;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use crate::command_result::{CommandError, CommandSuccess};

#[derive(Clone)]
pub struct GlobalSlashCommandDetails
{
    pub name: String,
    pub description: String,
    pub options: Vec<CommandOption>,
    //pub handler: fn(&ApplicationCommandInteraction, &Context, &Interaction) -> Result<(),String>
    pub handler: for<'a> fn(&'a ApplicationCommandInteraction, &'a Context, &'a Interaction) -> BoxFuture<'a, Result<CommandSuccess,CommandError>>
}

pub trait GetSlashCommandDetails {
    fn get_slash_command_details() -> GlobalSlashCommandDetails;
}

