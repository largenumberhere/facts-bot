use std::iter::zip;
use futures::{FutureExt, StreamExt};
use itertools::{chain, izip};
use crate::global_slash_command::{CommandError, CommandSuccess, GetSlashCommandDetails, GlobalSlashCommandDetails, ToCommandResult};
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::Interaction;
use serde::{Deserialize, Serialize};
use crate::{bot, CONTEXT_COMMANDS_LIST, SLASH_COMMANDS_LIST};

pub struct Help{}

impl GetSlashCommandDetails for Help{
    fn get_slash_command_details() -> GlobalSlashCommandDetails {
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

    let slash_names = SLASH_COMMANDS_LIST.iter().map(|f| &f.name);
    let slash_descriptions = SLASH_COMMANDS_LIST.iter().map(|f| &f.description);

    let context_names = CONTEXT_COMMANDS_LIST.iter().map(|f| &f.name);
    let context_descriptions = CONTEXT_COMMANDS_LIST.iter().map(|f| &f.help_description);

    response.push_str("**This bot is a utility bot and can complete many different tasks.\n here is a list of /commands:**\n");
    let slash_command_info:Vec<_> = zip(slash_names, slash_descriptions).collect();
    for (name,description) in slash_command_info {
            response.push_str(format!("{}\t-\t{} \n",name,description).as_str())
    }

    response.push_str("\n");
    response.push_str("**Here is a list of context menu commands (You can see this by right clicking on a user and selecting Apps):**\n");
    let context_command_into = zip(context_names, context_descriptions);
    for (name, description) in context_command_into{
        response.push_str(format!("{}\t-\t{}\n", name, description).as_str());
    }

    Ok(CommandSuccess::SuccessWithReply(response))
}