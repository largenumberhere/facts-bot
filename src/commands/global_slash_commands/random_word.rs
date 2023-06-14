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

pub struct RandomWordSlashCommand;

impl GetSlashCommandDetails for RandomWordSlashCommand {
    fn get_slash_command_details() -> GlobalSlashCommandDetails {
        GlobalSlashCommandDetails {
            name: "random_word".to_string(),
            description: "Get a random word and it's definition from api-ninjas.com".to_string(),
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

    let word = {
        let word_url: Url = "https://api.api-ninjas.com/v1/randomword"
            .parse()
            .to_command_result()?;

        let json = bot::HttpClient::https_get_json_with_headers(
            word_url,
            vec![("X-Api-Key", ninja_facts_key.as_str())],
        )
        .await
        .to_command_result()?;
        let response: RandomWordResponse =
            serde_json::from_str(json.as_str()).to_command_result()?;

        response.word
    };

    let dictionary_url = {
        let mut dictionary_url = "https://api.api-ninjas.com/v1/dictionary?word=".to_string();
        dictionary_url.write(word.as_str());
        let url: Result<Url, _> = dictionary_url.parse();
        let url = match url {
            Ok(v) => Some(v),
            Err(_) => None,
        };

        url
    };

    let definiton = match dictionary_url {
        Some(url) => {
            let response = bot::HttpClient::https_get_json_with_headers(
                url,
                vec![("X-Api-Key", ninja_facts_key.as_str())],
            )
            .await
            .to_command_result()?;
            let response: DictionaryResponse =
                serde_json::from_str(response.as_str()).to_command_result()?;
            match response.valid {
                true => Some(response.definition),
                false => None,
            }
        }
        None => None,
    };

    let reply = {
        let mut reply = String::new();
        reply.write("***Word:*** ").writeln(word.as_str());
        match definiton {
            Some(v) => {
                reply.write("***Definition:*** ").writeln(v.as_str());
            }
            None => {
                reply.write("*Could not find a definition*");
            }
        }

        reply
    };

    Ok(CommandSuccess::SuccessWithReply(reply))
}

#[derive(Serialize, Deserialize)]
struct RandomWordResponse {
    pub word: String,
}

#[derive(Serialize, Deserialize)]
struct DictionaryResponse {
    pub definition: String,
    pub word: String,
    pub valid: bool,
}
