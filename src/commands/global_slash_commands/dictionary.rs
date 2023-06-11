

use futures::FutureExt;
use lngh_strings::{WriteLnStringExt, WriteStringExt};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use serenity::builder::CreateApplicationCommandOption;
use serenity::client::Context;

use serenity::model::application::command::CommandOptionType;
use serenity::model::application::interaction::application_command::{ApplicationCommandInteraction};
use serenity::model::prelude::interaction::Interaction;
use crate::bot;
use crate::command_result::{CommandError, CommandSuccess, ToCommandResult};
use crate::global_slash_command::{get_string_option, GetSlashCommandDetails, GlobalSlashCommandDetails};

pub struct DictionarySlashCommand;

impl GetSlashCommandDetails for DictionarySlashCommand {
    fn get_slash_command_details() -> GlobalSlashCommandDetails {
        GlobalSlashCommandDetails{
            name: "dictionary".to_string(),
            description: "Get a dictionary definition for any word using https://api-ninjas.com/api/dictionary".to_string(),
            options:
                vec![
                    CreateApplicationCommandOption::default()
                        .name("word")
                        .kind(CommandOptionType::String)
                        .required(true)
                        .description("The world to search this dictionary for")
                        .to_owned()

                ],
            force_command_update:None,
            handler: |a,b,c| handle(a,b,c).boxed()
        }
    }
}


async fn handle(command_interaction: &ApplicationCommandInteraction, _context: &Context, _interaction: &Interaction) -> Result<CommandSuccess,CommandError>{
    let option = get_string_option(command_interaction)?;
    let string = option.trim();
    let address = "https://api.api-ninjas.com/v1/dictionary?word=";
    let url: Url = format!("{}{}", address, &string).parse().to_command_result()?;

    let token = bot::get_token_from("api-ninjas-com-key.file".to_string()).await.to_command_result()?;
    let json = bot::HttpClient::https_get_json_with_headers(url,vec![( "X-Api-Key",token.as_str())]).await.to_command_result()?;
    let response: DictionaryResponse = serde_json::from_str(json.as_str()).to_command_result()?;

    if !response.valid{
       return  Ok( CommandSuccess::SuccessWithReply(format!("'{}' not found in dictionary", string)));
    }

    let mut reply = String::new();
    reply.write("***Word:*** ").writeln(response.word.as_str())
        .writeln("***Definition:***")
        .writeln("```")
        .write(response.definition.as_str())
        .writeln("```");


    Ok(CommandSuccess::SuccessWithReply(reply))
}


#[derive(Serialize, Deserialize)]
struct DictionaryResponse {
    pub definition: String,
    pub word: String,
    pub valid: bool,
}