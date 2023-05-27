use std::iter::zip;
use futures::FutureExt;
use crate::global_slash_command::{CommandError, CommandSuccess, GetCommandDetails, GlobalSlashCommandDetails, ToCommandResult};
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::Interaction;
use serde::{Deserialize, Serialize};
use crate::{bot, COMMANDS_LIST};

pub struct Help{}

impl GetCommandDetails for Help{
    fn get_command_details() -> GlobalSlashCommandDetails {
        GlobalSlashCommandDetails{
            name:"help".to_string(),
            description:"Shows all the available commands".to_string(),
            options: vec![],
            handler: |command_interaction, context, interaction| handler(command_interaction, context, interaction).boxed()
        }
    }
}
async fn handler(command_interaction: &ApplicationCommandInteraction, context: &Context, interaction: &Interaction) -> Result<CommandSuccess, CommandError> {
    let mut response = String::new();
    let names = COMMANDS_LIST.iter().map(|f| &f.name);
    let decriptions = COMMANDS_LIST.iter().map(|f|&f.description);
    let command_info = zip(names,decriptions);

    response.push_str("This bot is a utility bot and can complete many different tasks. See a list of commands bellow.\n\n");
    for (name,description) in command_info{
        response.push_str(format!("{}\t-\t{} \n",name,description).as_str())
        }
    Ok(CommandSuccess::SuccessWithReply(response))
}
