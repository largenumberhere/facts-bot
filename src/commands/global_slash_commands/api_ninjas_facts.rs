use futures::FutureExt;
// use hyper::{Uri};
use serde::{Deserialize, Serialize};
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::Interaction;
use crate::bot;
use crate::global_slash_command::{GetSlashCommandDetails, GlobalSlashCommandDetails};
use std::string::String;
use crate::command_result::{CommandError, CommandSuccess, ToCommandResult, ToCommandResultWith};

pub struct ApiNinjasFacts{}

impl GetSlashCommandDetails for ApiNinjasFacts{
    fn get_slash_command_details() -> GlobalSlashCommandDetails {
        GlobalSlashCommandDetails{
            name: "facts".to_string(),
            description: "Get a random fact from https://api-ninjas.com/api/facts".to_string(),
            options: vec![],
            handler: |command_interaction, context, interaction| handler(command_interaction, context, interaction).boxed(),
            force_command_update: None,
        }
    }
}

async fn handler(_command_interaction: &ApplicationCommandInteraction, _context: &Context, _interaction: &Interaction) -> Result<CommandSuccess, CommandError> {
    let uri: reqwest::Url = "https://api.api-ninjas.com/v1/facts?limit=1".parse().to_command_result_with("url failed to convert")?;
    let ninja_facts_key = bot::get_token_from("api-ninjas-com-key.file".to_string()).await.to_command_result_with("Failed to get ninja facts key")?;

    let json = bot::HttpClient::https_get_json_with_headers(uri, vec![("X-Api-Key",ninja_facts_key.as_str())]).await.to_command_result_with("Http request with headers failed")?;

    let facts: Vec<FactReply> = serde_json::from_str(json.as_str()).to_command_result()?;
    let fact_reply = facts.get(0).ok_or_else(||"Failure to get fact 0".to_string()).to_command_result()?;

    Ok(CommandSuccess::SuccessWithReply(fact_reply.fact.to_string()))
}



#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FactReply {
    pub fact: String,
}

