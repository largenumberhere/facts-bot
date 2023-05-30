use std::any::Any;
use std::error::Error;
use std::future::Future;
use futures::future::ok;
use hyper::body::HttpBody;
use hyper::{Body, Method, Request};
use serenity::{async_trait};
use serenity::client::{Context, EventHandler};
use serenity::futures::StreamExt;
use serenity::http::Http;
use serenity::model::application::command::{Command, CommandType};
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



        // //try to add a context menu command!
        //
        // let command = Command::create_global_application_command(&context.http, |c| {
        //     c.kind(serenity::model::prelude::command::CommandType::User);
        //     c.name("hello world!");
        //     c.description("say hello world!")
        // });
        // let command = command.await;
        // if let Err(e) = command{
        //     panic!("{:#?}",e);
        // }
        // ////////////////////////////////////////





        println!("Connected as '{}'",bot_data.user.name);
        let mut new_command_results = Vec::new();
        let mut failed_commands = 0;


        let current_commands = Command::get_global_application_commands(&context.http).await.unwrap();
        let current_commands = current_commands.iter();


        for new_command in self.commands.iter(){
            if current_commands.clone().find(|c| c.name == new_command.name).is_some() {
                println!("command with name '{}' already found. Not registering it", new_command.name);
                continue;
            }


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


        //let all_commands = Command::get_global_application_commands(&context.http).await;
        // let all_commands = match all_commands {
        //     Ok(v) => {
        //         Some(v)
        //     }
        //     Err(e) => {
        //         eprintln!("Failed to fetch commands! Ignoring {:?}",e);
        //         None
        //     }
        // };


        //cleanup any old commands

        let old_commands = {
            let mut old_commands = Vec  ::new();

            // if let Some(ref all_commands) = current_commands {
                for command in current_commands.clone() {
                    match current_commands.clone().find(|ac| ac.id == command.id) {
                        Some(_v) => {},
                        None => {
                            old_commands.push(command)
                        }
                    }
                }
            //}

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
        match interaction {
            Interaction::ApplicationCommand(ref c) =>{
                handle_command_interaction(self,&context,&interaction, &c).await;
            }
            _=>{}
        }

    }

}

async fn handle_command_interaction(command_deals: &CommandsDetails, context: &Context, interaction: &Interaction, command: &ApplicationCommandInteraction){
    let name = command.user.name.clone();
    let discriminator = command.user.discriminator;
    println!("interaction received from {name}:{discriminator}");

    let command_name_requested = command.data.name.as_str();
    let bot_command = command_deals.commands.iter().find(|c| c.name.as_str() == command_name_requested);
    let slash_command = match bot_command {
        Some(v) => v,
        None => {
            command.quick_reply("not a valid command!".to_string(), &context.http).await;
            return;
        }
    };

    let command_processing_result = (slash_command.handler)(command, &context, &interaction).await;
    match command_processing_result {
        Ok(v) => {
            match v {
                CommandSuccess::Success => {},
                CommandSuccess::SuccessWithReply(e) => {
                    command.quick_reply(e, &context.http).await;
                }
            }
        }

        Err(e) => {
            match e {
                CommandError::InvalidUserInputError(e) => {
                    command.quick_reply(format!(":( Sorry we couldn't parse your data because: {}", e), &context.http).await;
                },
                CommandError::InternalError(e) => {
                    eprintln!("Failed to process command!'{e}'\n    Command_name{}\n    Command:{:?}\n  Intreraction:{:#?}  \nuser:{}:{}    \n user_id:{}", command.data.name, command, interaction, command.user.name, command.user.discriminator, command.user.id);
                    command.quick_reply(format!(":( sorry, your request failed because: {}", e), &context.http).await;
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

pub async fn get_token() -> Result<String, std::io::Error>{
    let file_contents = tokio::fs::read_to_string("./discord.file").await?;
    Ok(file_contents)
}

pub async fn get_token_from(fileName: String) -> Result<String, std::io::Error> {
    //Is reading the file at runtime more secure? Idk?? I'll come back to this later
    let string = tokio::fs::read_to_string(&fileName).await?;
    Ok(string)
    //std::fs::read_to_string(fileName.clone()).expect(&*format!("Could not find the file {}. An api key was expected to be in there", &fileName))
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

    pub async fn https_get_json_with_headers(uri: hyper::Uri, headers: Vec<(&'static str, &str)>) -> Result<String, Box<dyn Error>>{
        let https = hyper_tls::HttpsConnector::new();
        let client = hyper::client::Client::builder().build::<_, hyper::Body>(https);

        let mut req = Request::builder()
            .method(Method::GET)
            .uri(uri);
        // let mut headers_ =  req.headers_mut().unwrap();
        let mut headers_mut = req.headers_mut().ok_or("failed to grab headers".to_string())?;

        for (key, value) in headers{
            headers_mut.append(key,value.parse()?);
        }

        let req = req.body(Body::from(""))?;

        let mut response = client.request(req).await?;

        let mut buffer = Vec::new();

        while let Some(next) = response.body_mut().data().await {
            let chunk = next?;
            buffer.extend_from_slice(chunk.as_ref());
        }
        let json = String::from_utf8(buffer)?;
        Ok(json)
    }



}

