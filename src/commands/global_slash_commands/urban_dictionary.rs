use futures::FutureExt;
use lngh_strings::{WriteDebugStringExt, WriteLnDebugStringExt, WriteLnStringExt, WriteStringExt};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use serenity::builder::CreateApplicationCommandOption;
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::Interaction;
use serenity::model::prelude::command::CommandOptionType;
use crate::bot;
use crate::command_result::{CommandError, CommandSuccess, ToCommandResult};
use crate::global_slash_command::{get_string_option, GetSlashCommandDetails, GlobalSlashCommandDetails};

pub struct UrbanDictionaryCommand;

impl GetSlashCommandDetails for UrbanDictionaryCommand {
    fn get_slash_command_details() -> GlobalSlashCommandDetails {
        GlobalSlashCommandDetails{
            description: "Get the urban dictioanry definition of a word from https://api.urbandictionary.com/".to_string(),
            options: vec![
                CreateApplicationCommandOption::default()
                    .name("word")
                    .description("The word or phrase to search urban dictionary for")
                    .kind(CommandOptionType::String)
                    .required(true)
                    .to_owned()
            ],
            name: "urbandictionary".to_string(),
            force_command_update: None,
            handler: |command_interaction, context, interaction|    handler(command_interaction, context, interaction).boxed(),
        }
    }
}

async fn handler(command_interaction: &ApplicationCommandInteraction, context: &Context, interaction: &Interaction) -> Result<CommandSuccess,CommandError> {
    let option = get_string_option(command_interaction)?;
    let url: Url = format!("https://api.urbandictionary.com/v0/define?term={}", option).parse().to_command_result()?;
    let json = bot::HttpClient::https_get_json(url).await.to_command_result()?;

    let response: UrbanDictionaryResponse = serde_json::from_str(json.as_str()).to_command_result()?;
    //println!("{:#?}", response);



    let mut reply = String::new();
    reply.write("***Top definition for ").write(option.as_str()).writeln("***");



    let definition = match response.list.first() {
        None => {
            return Ok(CommandSuccess::SuccessWithReply(format!("No entry found for {}", option)));
        }
        Some(v) => {v}
    };
    {
        let time = chrono::NaiveDateTime::parse_from_str(definition.written_on.as_str(), "%Y-%m-%dT%H:%M:%S%.fZ");
        let time = match time {
            Ok(v) => v,
            Err(e) =>{
                return  Err(CommandError::InternalError(format!("failed to convert time '{}' becuase of parsing error '{:#?}'", definition.written_on , e)));
            }
        };

        let author = {
            if  definition.author.len() == 0{
                "[Unknown]"
            }
            else { //Bug: it refuses to compile if this if statement is not used
                definition.author.as_str()
            }
        };


        reply.writeln("```")
            .writeln(definition.definition.as_str())
            .writeln("```")
            .writeln("***Example:***")
            .writeln("```")
            .writeln(definition.example.as_str())
            .writeln("```")
            .write("***By*** ").write(author).write(" ").write("<t:").write(time.timestamp().to_string().as_str()).writeln(">")
            .write("Votes ").write(definition.thumbs_up.to_string().as_str()).write(" | ").writeln(definition.thumbs_down.to_string().as_str())
            .write(definition.permalink.as_str());
    }

    //max len 2000
    let reply_truncate_message = "\n ... *(discord message length limit reached)*";
    let message_len = reply_truncate_message.len();
    let max_space_availible = 2000-message_len;

    if message_len> max_space_availible {
        loop {
            let len = reply.len();
            if len > max_space_availible {
                reply.pop();
            } else {
                break;
            }
        }

        reply.write(reply_truncate_message);
    }




    Ok(CommandSuccess::SuccessWithReply(reply))
}

#[derive(Serialize, Deserialize, Debug)]
struct UrbanDictionaryDefinition {
    pub definition: String,
    pub permalink: String,
    pub thumbs_up: i64,
    pub author: String,
    pub word: String,
    pub defid: i64,
    pub current_vote: String, ///what the hell is this for? Why is it empty???
    pub written_on: String,
    pub example: String,
    pub thumbs_down: i64,
}

#[derive(Serialize, Deserialize, Debug)]
struct UrbanDictionaryResponse {
    //#[serde(rename = "list")]
    pub list: Vec<UrbanDictionaryDefinition>,
}