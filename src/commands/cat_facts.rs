use serenity::async_trait;
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::Interaction;
use crate::bot::QuickReply;
use crate::global_slash_command::{GetCommandDetails, GlobalSlashCommandDetails};
use crate::TOKIO_RUNTIME;

pub struct CatFactsCommand{}

impl GetCommandDetails for CatFactsCommand{
    fn get_command_details() -> GlobalSlashCommandDetails {
        return GlobalSlashCommandDetails {
            name: "cat_facts".to_string(),
            description: "Get daily cat facts from the project at https://alexwohlbruck.github.io/cat-facts/".to_string(),
            options: Vec::new().into(),
            handler
        }
    }
}

fn handler(command_interaction: &ApplicationCommandInteraction, context: &Context, interaction: &Interaction) -> Result<(),String>{
    let cat_info = "cats!";
    TOKIO_RUNTIME.block_on(
        command_interaction.quick_reply(cat_info.to_string(),&context.http)
    );

    //tokio::spawn(command_interaction.quick_reply(cat_info.to_string(), &context.http));
    Ok(())
}
