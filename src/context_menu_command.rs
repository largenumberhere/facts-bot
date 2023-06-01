use futures::future::BoxFuture;
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::Interaction;
use crate::global_slash_command::{CommandError, CommandSuccess};

#[derive(Clone)]
pub struct ContextMenuCommandDetails{
    pub name: String,
    pub help_description: String,
    pub handler: for<'a> fn(&'a ApplicationCommandInteraction, &'a Context, &'a Interaction) -> BoxFuture<'a, Result<CommandSuccess,CommandError>>
}

pub trait GetContextMenuCommandDetails{
    fn get_context_menu_command_details() -> ContextMenuCommandDetails;
}