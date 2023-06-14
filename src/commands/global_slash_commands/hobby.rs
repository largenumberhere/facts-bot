use crate::bot;
use crate::command_result::{CommandError, CommandSuccess, ToCommandResult};
use crate::global_slash_command::{GetSlashCommandDetails, GlobalSlashCommandDetails};
use futures::FutureExt;
use lngh_strings::{WriteLnStringExt, WriteStringExt};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::Interaction;

pub struct HobbySlashCommand;

impl GetSlashCommandDetails for HobbySlashCommand {
    fn get_slash_command_details() -> GlobalSlashCommandDetails {
        GlobalSlashCommandDetails {
            name: "hobby".to_string(),
            description: "Get a random hobby suggestion from https://api-ninjas.com/api/hobbies"
                .to_string(),
            options: vec![],
            force_command_update: None,
            handler: |command_interaction, context, interaction| {
                handler(command_interaction, context, interaction).boxed()
            },
        }
    }
}

async fn handler(
    _command_interaction: &ApplicationCommandInteraction,
    _context: &Context,
    _interaction: &Interaction,
) -> Result<CommandSuccess, CommandError> {
    let ninja_facts_key = bot::get_token_from("api-ninjas-com-key.file".to_string())
        .await
        .to_command_result()?;

    let url: Url = "https://api.api-ninjas.com/v1/hobbies?category=general"
        .parse()
        .to_command_result()?;
    let json = bot::HttpClient::https_get_json_with_headers(
        url,
        vec![("X-Api-Key", ninja_facts_key.as_str())],
    )
    .await
    .to_command_result()?;
    let response: HobbyResponse = serde_json::from_str(json.as_str()).to_command_result()?;

    let reply = {
        let mut reply = String::new();
        reply
            .writeln("***Hobby suggestion***")
            .write("***Name:*** ")
            .writeln(response.hobby.as_str())
            .write("***Link*** ")
            .writeln(response.link.as_str());

        reply
    };

    Ok(CommandSuccess::SuccessWithReply(reply))
}

#[derive(Serialize, Deserialize)]
struct HobbyResponse {
    pub hobby: String,
    pub link: String,
    pub category: String,
}
