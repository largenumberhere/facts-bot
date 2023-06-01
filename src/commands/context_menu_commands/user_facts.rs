use futures::{FutureExt};
use crate::context_menu_command::{ContextMenuCommandDetails, GetContextMenuCommandDetails};
use serenity::client::Context;
use serenity::model::application::interaction::application_command::{ApplicationCommandInteraction, ResolvedTarget};
use serenity::model::application::interaction::{Interaction};
use crate::bot::{QuickReplyEphemeral};
use crate::command_result::{CommandError, CommandSuccess, ToCommandResult};

pub struct UserFactsContextCommand {}

impl GetContextMenuCommandDetails for UserFactsContextCommand {
    fn get_context_menu_command_details() -> ContextMenuCommandDetails {
        ContextMenuCommandDetails{
            name: "user_facts".to_string(),
            help_description: "shows a list of facts about a user".to_string(),
            handler: |command_interaction, context, interaction| handler(command_interaction, context, interaction).boxed(),
            force_command_update: None
        }
    }
}

async fn handler(command_interaction: &ApplicationCommandInteraction, context: &Context, _interaction: &Interaction) -> Result<CommandSuccess, CommandError> {
    let target_info = command_interaction.data.target().ok_or("Failed to fetch user info for the specified target!".to_string()).to_command_result()?;

    let (target_user, _partial_user_info) = match target_info {
        ResolvedTarget::User(user, b) => {
            (user, b)
        }
        ResolvedTarget::Message(_) => {
            return Err(CommandError::InvalidUserInputError("Context menu commands should always have a user id, not a message! How did you break this??".to_string()));
        }
        _ => {
            return  Err(CommandError::InvalidUserInputError("Unreachable...".to_string()));
        }
    };

    //let partial_user_info = _partial_user_info.ok_or("Failed to fetch information about the targeted user".to_string()).to_command_result()?;
    let user = context.http.get_user(target_user.id.0).await.to_command_result()?;

    let mut reply = String::new();
    reply.push_str(format!("Facts about the user '{}#{}':\n", user.name, user.discriminator).as_str());
    if user.bot{
        reply.push_str(format!("User is definitely a bot\n").as_str());
    }
    else {
        reply.push_str("User is probably not a bot\n");
    }
    reply.push_str(format!("The user's internal discord id is '{}'\n", user.id.0).as_str());
    match user.public_flags {
        None => {
            reply.push_str("Could not fetch the user's flags\n");
        }
        Some(flags) => {
            reply.push_str(format!("These flags are on the user's account: {:#?}\n", flags).as_str());
        }
    }

    let timestamp = user.created_at().unix_timestamp();
    reply.push_str(format!("The user was created at: <t:{}> (In your timezone), or {} unix time \n",timestamp, timestamp ).as_str());
    reply.push_str(format!("The user's avatar can be found at: {}\n", user.face()).as_str());
    match user.banner_url() {
        Some(v) =>{
            reply.push_str(format!("The user's banner can be found at: {}\n", v).as_str());
        },
        None=>{
            match user.accent_colour {
                Some(colour) =>{
                    reply.push_str( format!("The user has the banner colour hex #{}\n", colour.hex()).as_str())
                },
                None =>{
                    reply.push_str("The user has a default banner\n");
                }
            }
        }
    }




    //reply.push_str(format!("The user's nickname is: '{}'\n", user.nick_in().await.unwrap_or("none".to_string())).as_str());

    command_interaction.quick_reply_ephemeral(reply, &context.http).await;






    //command_interaction.quick_reply("aaa".to_string(), &context.http).await;

    Ok(CommandSuccess::Success)
}