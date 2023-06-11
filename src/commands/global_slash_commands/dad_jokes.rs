
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::Interaction;
use crate::bot;
use crate::command_result::{CommandError, CommandSuccess, ToCommandResultWith};
use crate::command_result::CommandSuccess::SuccessWithReply;
use crate::global_slash_command::{GetSlashCommandDetails, GlobalSlashCommandDetails};
use futures::FutureExt;

pub struct DadJokesCommand;
impl GetSlashCommandDetails for DadJokesCommand{
    fn get_slash_command_details() -> GlobalSlashCommandDetails {
        GlobalSlashCommandDetails{
            name: "dad_joke".to_string(),
            description: "get a dad joke from https://api-ninjas.com/api/dadjokes".to_string(),
            options: vec![],
            handler: |command_interaction, context, interaction|    handler(command_interaction, context, interaction).boxed(),
            force_command_update: None,
        }
    }
}

async fn handler(_command_interaction: &ApplicationCommandInteraction, _context: &Context, _interaction: &Interaction) -> Result<CommandSuccess, CommandError> {
    let ninja_facts_key = bot::get_token_from("api-ninjas-com-key.file".to_string()).await.to_command_result_with("failed to read api key from file on server")?;
    let url = "https://api.api-ninjas.com/v1/dadjokes?limit=1".parse().to_command_result_with("failed to parse dadjokes url")?;
    let reply = bot::HttpClient::https_get_json_with_headers(url, vec![("X-Api-Key", ninja_facts_key.as_str())]).await.to_command_result_with("failed to read http reply json")?;
    let jokes:Jokes = serde_json::from_str(reply.as_str()).to_command_result_with("response from server was unclear. The JSON was a mess")?;
    let joke = jokes.get(0).ok_or(()).to_command_result_with("there was no joke );")?;
    let joke = &joke.joke;

    Ok(SuccessWithReply(joke.to_string()))
}

use serde::{Serialize, Deserialize};

pub type Jokes = Vec<Joke>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Joke {
    pub joke: String,
}