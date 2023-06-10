use futures::future::BoxFuture;
use serenity::builder::CreateApplicationCommandOption;
use serenity::client::Context;
use serenity::json::Value;
use serenity::model::application::interaction::Interaction;
use serenity::model::prelude::command::CommandOption;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use crate::command_result::{CommandError, CommandSuccess};

#[derive(Clone)]
pub struct GlobalSlashCommandDetails
{
    pub name: String,
    pub description: String,
    pub options: Vec<CreateApplicationCommandOption>,
    //pub handler: fn(&ApplicationCommandInteraction, &Context, &Interaction) -> Result<(),String>
    pub handler: for<'a> fn(&'a ApplicationCommandInteraction, &'a Context, &'a Interaction) -> BoxFuture<'a, Result<CommandSuccess,CommandError>>,
    pub force_command_update: Option<()>
}

pub trait GetSlashCommandDetails {
    fn get_slash_command_details() -> GlobalSlashCommandDetails;
}

pub fn get_string_option(command_interaction: &ApplicationCommandInteraction) -> Result<String, CommandError>{
    let options = &command_interaction.data.options;
    if options.len()!= 1{
        return  Err(CommandError::InvalidUserInputError(format!("Exactly 1 option expcted for command reponse. {} given",options.len())));
    }

    let option0 = match options.first() {
        Some(v) => v,
        None => return Err(CommandError::InternalError(format!("First option did not exist but size was 1 waaaaa????! >~> options:'{:#?}'", options)))
    };

    let option0_value = match &option0.value {
        Some(v) => v,
        None => {
            return  Err(CommandError::InvalidUserInputError(format!("First option did not have a type aaaaaaaaaaa. options:'{:#?}'\noption0:{:#?}", options, option0)));
        }
    };

    let option0_as_string = match option0_value {
        Value::String(s) => {s}
        _=>{
            return  Err(CommandError::InvalidUserInputError(format!("First option must be of type String. Received: {}", option0_value)));
        }
    };

    Ok(option0_as_string.to_owned())
}