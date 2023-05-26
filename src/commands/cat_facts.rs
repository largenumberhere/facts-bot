use serenity::async_trait;
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::Interaction;
use crate::bot::QuickReply;
use crate::global_slash_command::{CommandHandler, GetCommandDetails, GlobalSlashCommandDetails};

pub struct CatFactsCommand{}

impl GetCommandDetails for CatFactsCommand{
    fn get_command_details(&self) -> GlobalSlashCommandDetails {
        return GlobalSlashCommandDetails {
            name: "cat_facts".to_string(),
            description: "Get daily cat facts from the project at https://alexwohlbruck.github.io/cat-facts/".to_string(),
            options: Vec::new(),
            // request_handler: |command_interaction, context, interaction|{
            //
            //
            //     let cat_info = "cats!";
            //
            //     command_interaction.quick_reply(cat_info.to_string(),&context.http);
            //
            //     Ok(())
            // },



        }
    }
}

#[async_trait]
impl CommandHandler for CatFactsCommand{
    async fn handle_request(&self ,command_interaction: &ApplicationCommandInteraction, context: &Context, interaction: &Interaction) -> Result<(), String> {
        let cat_info = "cats!";
        command_interaction.quick_reply(cat_info.to_string(), &context.http).await;
        Ok(())
    }
}

