use std::future::Future;
use serenity::async_trait;
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::Interaction;
use crate::bot::QuickReply;
use crate::global_slash_command::{GetCommandDetails, GlobalSlashCommandDetails};

pub struct CatFactsCommand{}

impl<T: Future<Output = Result<(), String>>> GetCommandDetails<T> for CatFactsCommand {
    fn get_command_details() -> GlobalSlashCommandDetails<TAsyncResult>
        where TAsyncResult:  Future<Output = Result<(), String>>
    {
        return GlobalSlashCommandDetails {
            name: "cat_facts".to_string(),
            description: "Get daily cat facts from the project at https://alexwohlbruck.github.io/cat-facts/".to_string(),
            options: Vec::new().into(),
            handler: handler
        }
    }
}

async fn handler(command_interaction: &ApplicationCommandInteraction, context: &Context, interaction: &Interaction) -> Result<(),String>{
    let cat_info = "cats!";

    command_interaction.quick_reply(cat_info.to_string(),&context.http).await.unwrap();

    //tokio::spawn(command_interaction.quick_reply(cat_info.to_string(), &context.http));
    Ok(())
}
