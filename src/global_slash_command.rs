use serenity::async_trait;
use serenity::client::Context;
use serenity::model::application::interaction::Interaction;
use serenity::model::prelude::command::CommandOption;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;

#[derive(Clone)]
pub struct GlobalSlashCommandDetails
{
    pub name: String,
    pub description: String,
    pub options: Vec<CommandOption>,
}

#[async_trait]
pub trait CommandHandler: Send + Sync{
    async fn handle_request(&self, command_interaction: &ApplicationCommandInteraction, context: & Context, interaction: &Interaction) -> Result<(),String>;
}

pub trait GetCommandDetails {
    fn get_command_details(&self) -> GlobalSlashCommandDetails where Self: Sized;
}


#[async_trait]
pub trait GlobalSlashCommand : GetCommandDetails + CommandHandler + Sync + Send {}

