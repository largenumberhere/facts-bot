use std::error::Error;
use std::future::Future;
use hyper::body::HttpBody;
use serenity::{async_trait};
use serenity::client::{Context, EventHandler};
use serenity::futures::StreamExt;
use serenity::http::Http;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::Interaction;
use serenity::model::application::interaction::InteractionResponseType::ChannelMessageWithSource;
use serenity::model::channel::MessageType::ContextMenuCommand;
use serenity::model::gateway::Ready;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::GatewayIntents;
use crate::global_slash_command::{CommandError, CommandSuccess, GlobalSlashCommandDetails};

struct CommandsDetails {
    commands: Vec<GlobalSlashCommandDetails>
}

#[async_trait]
impl EventHandler for CommandsDetails {
    async fn ready(&self, context: Context, bot_data: Ready) {
        println!("Connected as '{}'",bot_data.user.name);

        let mut new_command_results = Vec::new();
        let mut failed_commands = 0;


        for new_command in self.commands.iter(){
            let result = Command::create_global_application_command(&context.http, |command_builder|{
                command_builder.name(&new_command.name)
                    .description(&new_command.description);

                for option in new_command.options.iter(){
                    command_builder.create_option(|option_builder|{
                        option_builder.name(&option.name)
                            .description(&option.description)
                            .kind(option.kind)
                            .required(option.required)
                    });
                }

                command_builder
            }).await;

            match result {
                Ok(c) =>{
                    println!("command created! '{}'",c.name);
                    new_command_results.push(c);
                },
                Err(ref e)=> {
                    failed_commands+=1;
                    eprintln!("failed to create command!\ncommand_result:{:#?}\nerror:{}. {} command(s) failed to register", result, e,failed_commands);
                }
            }
        }


        //cleanup any old commands
        let all_commands = Command::get_global_application_commands(&context.http).await;
        let all_commands = match all_commands {
            Ok(v) => {
                Some(v)
            }
            Err(e) => {
                eprintln!("Failed to fetch commands! Ignoring {:?}",e);
                None
            }
        };



        let old_commands = {
            let mut old_commands = Vec  ::new();

            if let Some(ref all_commands) = all_commands {
                for command in all_commands {
                    match new_command_results.iter().find(|ac| ac.id == command.id) {
                        Some(_v) => {},
                        None => {
                            old_commands.push(command)
                        }
                    }
                }
            }

            old_commands
        };

        for unused_command in old_commands {
            match Command::delete_global_application_command(&context.http, unused_command.id).await {
                Err(e) => eprintln!("Failed to delete command with id: '{}', name: '{}'", unused_command.id, unused_command.name),
                Ok(v) => println!("Deleted command with id: '{}', name: '{}',", unused_command.id, unused_command.name)
            }
        }
    }

    async fn interaction_create(&self ,context: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(ref command) = interaction {
            let name = command.user.name.clone();
            let discriminator = command.user.discriminator;
            println!("interaction received from {name}:{discriminator}");

            let command_name_requested = command.data.name.as_str();
            let bot_command = self.commands.iter().find(|c| c.name.as_str() == command_name_requested );
            let slash_command = match bot_command {
                Some(v) => v,
                None =>{
                    command.quick_reply("not a valid command!".to_string(), &context.http).await;
                    return;
                }
            };

            let command_processing_result = (slash_command.handler)(command, &context, &interaction).await;
            match command_processing_result{
                Ok(v) =>{
                    match v {
                        CommandSuccess::Success=>{},
                        CommandSuccess::SuccessWithReply(e)=>{
                            command.quick_reply(e,&context.http).await;
                        }

                    }
                }

                Err(e)=>{
                    match e {
                        CommandError::InvalidUserInputError(e) =>{
                            command.quick_reply(format!(":( Sorry we couldn't parse your data because: {}",e),&context.http).await;
                        },
                        CommandError::InternalError(e) =>{
                            eprintln!("Failed to process command!'{e}'\n    Command_name{}\n    Command:{:?}\n  Intreraction:{:#?}  \nuser:{}:{}    \n user_id:{}", command.data.name, command, interaction, command.user.name,command.user.discriminator,command.user.id);
                            command.quick_reply(format!(":( sorry, your request failed because: {}",e),&context.http).await;
                        }
                    }
                }


            }
        }

    }

}


pub async fn start(bot_token: String, intents: GatewayIntents, commands: Vec<GlobalSlashCommandDetails>) -> Result<(),Box<dyn Error>> {

    // let cmd = *commands.iter().clone().collect::<Vec<_>>();
    let mut client =serenity::client::Client::builder(bot_token, intents)
        .event_handler(CommandsDetails{
            commands
        }).await?;
    client.start().await?;
    Ok(())
}

pub async fn get_token() -> String{
    std::fs::read_to_string("./discord.file").expect("./discord.file should contain a bot token")
}

pub async fn get_token_from(fileName: String) -> String{
    std::fs::read_to_string(fileName.clone()).expect(&*format!("could not find {}", &fileName))
}

#[async_trait]
pub trait QuickReply{
    async fn quick_reply(&self, text: String, http: &Http);
}


#[async_trait]
impl QuickReply for &ApplicationCommandInteraction{
    async fn quick_reply(&self, text: String, http: &Http) {
        let result = self.create_interaction_response(http,|r|{
            r.kind(ChannelMessageWithSource)
                .interaction_response_data(|c|c.content(text))
        }).await;

        match result {
            Ok(_) =>{
                println!("Responded to {}:{}",self.user.name, self.user.discriminator);
            },
            Err(e) => {
                eprintln!("Faied to respond to {}:{}, because {:#?}",self.user.name, self.user.discriminator, e);
            }
        }
    }
}

pub struct HttpClient{}

impl HttpClient{
    pub async fn http_get_json(uri: hyper::Uri) -> Result<String, Box<dyn Error>>{
        let client = hyper::client::Client::new();
        let mut connection = client.get(uri).await?;
        let mut buffer = Vec::new();

        while let Some(next) = connection.body_mut().data().await {
            let chunk = next?;
            buffer.extend_from_slice(chunk.as_ref());
        }

        Ok(String::from_utf8(buffer)?)
    }


    pub async fn https_get_json(uri: hyper::Uri) -> Result<String, Box<dyn Error>>{
        let https = hyper_tls::HttpsConnector::new();
        let client = hyper::client::Client::builder().build::<_, hyper::Body>(https);

        let mut connection = client.get(uri).await?;
        let mut buffer = Vec::new();

        while let Some(next) = connection.body_mut().data().await {
            let chunk = next?;
            buffer.extend_from_slice(chunk.as_ref());
        }

        Ok(String::from_utf8(buffer)?)
    }



}

