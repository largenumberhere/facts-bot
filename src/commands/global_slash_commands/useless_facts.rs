use futures::FutureExt;
use crate::global_slash_command::{GetSlashCommandDetails, GlobalSlashCommandDetails};
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::Interaction;
use serde::{Deserialize, Serialize};
use crate::bot;
use crate::command_result::{CommandError, CommandSuccess, ToCommandResult};

pub struct UselessFactsCommand{}

impl GetSlashCommandDetails for UselessFactsCommand{
    fn get_slash_command_details() -> GlobalSlashCommandDetails {
        GlobalSlashCommandDetails{
            name: "useless_fact".to_string(),
            description: "Get a 'useless' fun fact from https://uselessfacts.jsph.pl/".to_string(),
            options: vec![],
            handler: |command_interaction, context, interaction| handler(command_interaction, context, interaction).boxed(),
            force_command_update: None,
        }
    }
}

async fn handler(_command_interaction: &ApplicationCommandInteraction, _context: &Context, _interaction: &Interaction) -> Result<CommandSuccess, CommandError>{
    let uri = "https://uselessfacts.jsph.pl/api/v2/facts/random".parse().to_command_result()?;
    let json = bot::HttpClient::https_get_json(uri).await.to_command_result()?;
    let result:UselessFactsResult =  serde_json::from_str(json.as_str()).to_command_result()?;

    Ok(CommandSuccess::SuccessWithReply(result.text))
}

#[derive(Serialize)]
#[derive(Deserialize)]
struct UselessFactsResult{
    id: String,
    text: String,
    source: String,
    source_url: String,
    language: String,
    permalink: String
}