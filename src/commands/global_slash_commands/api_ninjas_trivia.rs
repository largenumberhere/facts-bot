use futures::FutureExt;
// use hyper::Uri;
use crate::bot;
use crate::command_result::{CommandError, CommandSuccess, ToCommandResult};
use crate::global_slash_command::{GetSlashCommandDetails, GlobalSlashCommandDetails};
use serde::Deserialize;
use serde::Serialize;
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::Interaction;

pub struct ApiNinjasTrivia {}

impl GetSlashCommandDetails for ApiNinjasTrivia {
    fn get_slash_command_details() -> GlobalSlashCommandDetails {
        GlobalSlashCommandDetails {
            name: "trivia".to_string(),
            options: vec![],
            description:
                "Get a trivia question and answer from https://api.api-ninjas.com/v1/trivia"
                    .to_string(),
            handler: |command_interaction, context, interaction| {
                handler(command_interaction, context, interaction).boxed()
            },
            force_command_update: None,
        }
    }
}

async fn handler(
    _command_interaction: &ApplicationCommandInteraction,
    _context: &Context,
    _interaction: &Interaction,
) -> Result<CommandSuccess, CommandError> {
    let uri: reqwest::Url = "https://api.api-ninjas.com/v1/trivia?category=general"
        .parse()
        .to_command_result()?;
    let ninja_facts_key = bot::get_token_from("api-ninjas-com-key.file".to_string())
        .await
        .to_command_result()?;
    let json = bot::HttpClient::https_get_json_with_headers(
        uri,
        vec![("X-Api-Key", ninja_facts_key.as_str())],
    )
    .await
    .to_command_result()?;

    let trivia_responses: TriviaResponses =
        serde_json::from_str(json.as_str()).to_command_result()?;
    let trivia_response: &TriviaResponse = trivia_responses
        .get(0)
        .ok_or("failed to get value 0 from trivia response json")
        .to_command_result()?;

    let mut response = String::new();
    response.push_str(format!("Q: {}\n", trivia_response.question).as_str());
    response.push_str(
        format!(
            "A: ||{}|| (click on black box to show)",
            trivia_response.answer
        )
        .as_str(),
    );

    Ok(CommandSuccess::SuccessWithReply(response))
}

//there is normally just one of them
pub type TriviaResponses = Vec<TriviaResponse>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TriviaResponse {
    pub category: String,
    pub question: String,
    pub answer: String,
}
