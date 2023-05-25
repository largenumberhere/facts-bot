use serenity::client::Context;
use serenity::model::application::interaction::Interaction;
use serenity::model::prelude::command::CommandOption;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;

#[derive(Clone)]

pub struct GlobalSlashCommand {
    pub name: String,
    pub description: String,
    pub options: Vec<CommandOption>,
    pub request_handler: fn(&ApplicationCommandInteraction, &Context, &Interaction) -> Result<(),String>
}