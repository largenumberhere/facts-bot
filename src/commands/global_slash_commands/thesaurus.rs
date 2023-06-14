use crate::bot;
use crate::command_result::{CommandError, CommandSuccess, ToCommandResult};
use crate::global_slash_command::{
    get_string_option, GetSlashCommandDetails, GlobalSlashCommandDetails,
};
use futures::FutureExt;
use lngh_strings::{WriteLnStringExt, WriteStringExt};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use serenity::builder::CreateApplicationCommandOption;
use serenity::client::Context;
use serenity::model::application::command::CommandOptionType;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::Interaction;

pub struct ThesaurusSlashCommand;

impl GetSlashCommandDetails for ThesaurusSlashCommand {
    fn get_slash_command_details() -> GlobalSlashCommandDetails {
        GlobalSlashCommandDetails {
            name: "thesaurus".to_string(),
            description: "A book of synonyms. This one is not very good".to_string(),
            options: vec![{
                let mut option = CreateApplicationCommandOption::default();
                option
                    .name("word")
                    .description("the word to search for")
                    .kind(CommandOptionType::String)
                    .required(true);
                option
            }],
            force_command_update: None,
            handler: |command_interaction, context, interaction| {
                handler(command_interaction, context, interaction).boxed()
            },
        }
    }
}

async fn handler(
    command_interaction: &ApplicationCommandInteraction,
    _context: &Context,
    _interaction: &Interaction,
) -> Result<CommandSuccess, CommandError> {
    let option0 = get_string_option(command_interaction)?;

    let mut thesaurus_url = "https://api.api-ninjas.com/v1/thesaurus?word=".to_string();
    thesaurus_url.write(option0.trim());
    let thesaurus_url: Url = thesaurus_url.parse().to_command_result()?;

    let ninja_facts_key = bot::get_token_from("api-ninjas-com-key.file".to_string())
        .await
        .to_command_result()?;

    let json = bot::HttpClient::https_get_json_with_headers(
        thesaurus_url,
        vec![("X-Api-Key", ninja_facts_key.as_str())],
    )
    .await
    .to_command_result()?;
    let response: Result<DictionaryNormalResponse, _> = serde_json::from_str(json.as_str());
    let response = match response {
        Err(e) => {
            //try to get error from json then return
            if let Ok(v) = serde_json::from_str::<ThesaurusErrorResponse>(json.as_str()) {
                return Err(CommandError::InvalidUserInputError(format!(
                    "Thesaurus api error: {}",
                    v.error
                )));
            }

            return Err(CommandError::InternalError(format!(
                "Could not understand api response. Details: '{:?}'\nOffending json: '{}'",
                e, json
            )));
        }
        Ok(v) => v,
    };

    let reply = {
        let mut reply = String::new();
        reply
            .write("***Entry for***: ")
            .writeln(response.word.as_str())
            .write("***Synonyms***: ");

        if response.synonyms.len() > 0 {
            for synonym in response.synonyms {
                reply.write(" ").write(synonym.as_str()).write(",");
            }
        } else {
            reply.writeln("[None found]");
        }

        reply.writeln("").write("***Antonyms***: ");

        if response.antonyms.len() > 0 {
            for antonym in response.antonyms {
                reply.write(" ").write(antonym.as_str()).write(",");
            }
        } else {
            reply.writeln("[None found]");
        }

        reply.writeln("");

        reply
    };

    Ok(CommandSuccess::SuccessWithReply(reply))
}

#[derive(Serialize, Deserialize)]
struct ThesaurusErrorResponse {
    pub error: String,
}

#[derive(Serialize, Deserialize)]
struct DictionaryNormalResponse {
    pub word: String,
    pub synonyms: Vec<String>,
    pub antonyms: Vec<String>,
}
