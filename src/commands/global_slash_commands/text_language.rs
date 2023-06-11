use futures::FutureExt;
use lngh_strings::{WriteLnStringExt, WriteStringExt};
use serde::{Deserialize, Serialize};
use serenity::builder::CreateApplicationCommandOption;
use serenity::client::Context;
use serenity::model::application::command::CommandOptionType;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::Interaction;
use crate::bot;
use crate::command_result::{CommandError, CommandSuccess, ToCommandResult, ToCommandResultWith};
use crate::global_slash_command::{get_string_option, GetSlashCommandDetails, GlobalSlashCommandDetails};

pub struct TextLanguageSlashCommand;

impl GetSlashCommandDetails for TextLanguageSlashCommand{
    fn get_slash_command_details() -> GlobalSlashCommandDetails {
        GlobalSlashCommandDetails{
            name: "textlanguage".to_string(),
            options: vec![
                {
                    let option = CreateApplicationCommandOption::default()
                        .name("paragraph")
                        .description("A small sample of text to check the language (10 words min)")
                        .kind(CommandOptionType::String)
                        .to_owned();//Can't find better way to remove mut
                    option
                },
            ],
            description: "Check what language a sentence a paragraph written in. (At least 10 words!) by api-ninjas.com".to_string(),
            force_command_update: None,
            handler: |command_interaction, context, interaction| handler(command_interaction, context, interaction).boxed(),
        }
    }
}

async fn handler (command_interaction: &ApplicationCommandInteraction, context: &Context, interaction: &Interaction) -> Result<CommandSuccess, CommandError> {
    let option0 = get_string_option(command_interaction)?;
    let option_trimmed = option0.trim();

    if option0.len()> 1000 {
        return Err(CommandError::InvalidUserInputError("Sampple is too long. Max 2000 characters allowed".to_string()));
    }

    let words = option0.split_whitespace().count()+1;
    if words < 10 {
        return Err(CommandError::InvalidUserInputError("Sample is too short. Min 10 characters.".to_string()))
    }

    let mut url = "https://api.api-ninjas.com/v1/textlanguage?text=".to_string();
    url.write(option0.as_str());

    let url = url.parse().to_command_result()?;

    let ninja_facts_key = bot::get_token_from("api-ninjas-com-key.file".to_string()).await.to_command_result()?;


    let json = bot::HttpClient::https_get_json_with_headers(url, vec![("X-Api-Key",ninja_facts_key.as_str())]).await.to_command_result()?;

    let valid_response: Result<TextResponse,_> = serde_json::from_str(json.as_str());
    let valid_response = match valid_response {
        Ok(v) => v,
        Err(e1) =>{
            let error_response: Result<TextErrorResponse, _> = serde_json::from_str(json.as_str());
            let error_response =  match error_response {
                Ok(v) =>v,
                Err(e2) => {
                    let response = format!(
                        "Failed to parse json as error form or valid form!\n\
                        TextResponse parse error: '{:#?}'\n\
                        TextErrorResponse parse error: '{:#?}'\n\
                        Offending json: '{}'",
                        e1,
                        e2,
                        json
                    );
                    return Err(CommandError::InternalError(response));
                }
            };
            return Err(CommandError::InvalidUserInputError(format!("Api error: '{}'",error_response.error)))
        }
    };

    let reply ={
        let mut reply = String::new();
        reply.write("***Language name:*** ").writeln(valid_response.language.as_str())
            .write("***Language Id (ISO):*** ").writeln(valid_response.iso.as_str());

        reply
    };

    Ok(CommandSuccess::SuccessWithReply(reply))
}

#[derive(Serialize, Deserialize)]
struct TextErrorResponse {
    pub error: String,
}

#[derive(Serialize, Deserialize)]
struct TextResponse {
    pub iso: String,
    pub language: String,
}