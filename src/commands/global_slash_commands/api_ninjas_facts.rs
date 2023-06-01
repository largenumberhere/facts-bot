use futures::FutureExt;
use hyper::body::HttpBody;
use hyper::{Body, HeaderMap, Method, Request, Uri};
use hyper::http::HeaderValue;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::Interaction;
use crate::bot;
use crate::global_slash_command::{GetSlashCommandDetails, GlobalSlashCommandDetails};
use std::string::String;
use crate::command_result::{CommandError, CommandSuccess, ToCommandResult};

pub struct ApiNinjasFacts{}

impl GetSlashCommandDetails for ApiNinjasFacts{
    fn get_slash_command_details() -> GlobalSlashCommandDetails {
        GlobalSlashCommandDetails{
            name: "facts".to_string(),
            description: "Get a random fact from https://api-ninjas.com/api/facts".to_string(),
            options: vec![],
            handler: |command_interaction, context, interaction| handler(command_interaction, context, interaction).boxed()
        }
    }
}

async fn handler(command_interaction: &ApplicationCommandInteraction, context: &Context, interaction: &Interaction) -> Result<CommandSuccess, CommandError> {
    let uri: Uri = "https://api.api-ninjas.com/v1/facts?limit=1".parse().to_command_result()?;
    let ninja_facts_key = bot::get_token_from("api-ninjas-com-key.file".to_string()).await;

    let json = bot::HttpClient::https_get_json_with_headers(uri, vec![("X-Api-Key",ninja_facts_key.to_command_result()?.as_str())]).await.to_command_result()?;

    let facts: Vec<FactReply> = serde_json::from_str(json.as_str()).to_command_result()?;
    let factReply = facts.get(0).ok_or_else(||"Failure to get fact 0".to_string()).to_command_result()?;

    Ok(CommandSuccess::SuccessWithReply(factReply.fact.to_string()))
}


pub type FactsReply = Vec<FactReply>;
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FactReply {
    pub fact: String,
}

