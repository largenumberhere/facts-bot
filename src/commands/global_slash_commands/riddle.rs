use futures::FutureExt;
use lngh_strings::{WriteLnStringExt, WriteStringExt};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::Interaction;
use crate::bot;
use crate::command_result::{CommandError, CommandSuccess, ToCommandResult, ToCommandResultWith};
use crate::global_slash_command::{GetSlashCommandDetails, GlobalSlashCommandDetails};

pub struct RiddleSlashCommand;

impl GetSlashCommandDetails for RiddleSlashCommand{
    fn get_slash_command_details() -> GlobalSlashCommandDetails {
        GlobalSlashCommandDetails{
            name: "riddle".to_string(),
            description: "Get a riddle question and answer".to_string(),
            options: vec![],
            force_command_update: None,
            handler: |command_interaction, context, interaction| handler(command_interaction, context, interaction).boxed(),
        }
    }
}

async fn handler (_command_interaction: &ApplicationCommandInteraction, _context: &Context, _interaction: &Interaction) -> Result<CommandSuccess, CommandError> {
    let ninja_facts_key = bot::get_token_from("api-ninjas-com-key.file".to_string()).await.to_command_result()?;
    let url:Url = "https://api.api-ninjas.com/v1/riddles".parse().to_command_result()?;

    let json = bot::HttpClient::https_get_json_with_headers(url, vec![("X-Api-Key",ninja_facts_key.as_str())]).await.to_command_result()?;
    let response: TriviaResults = serde_json::from_str(json.as_str()).to_command_result_with("failed to serialize response")?;
    let response0 = &response[0];

    let reply = {
      let mut reply = String::new();
        reply.write("***Title:***") .writeln(response0.title.as_str())
            .write("***Q:*** ").writeln(response0.question.as_str())
            .write("***A:*** ||").writeln(response0.answer.as_str()).write("||");
        reply
    };


    Ok(CommandSuccess::SuccessWithReply(reply))
}

type TriviaResults =  [TriviaItems; 1];
#[derive(Serialize, Deserialize)]
struct TriviaItems {
    pub title: String,
    pub question: String,
    pub answer: String,
}