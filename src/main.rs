use crate::commands::context_menu_commands;
use crate::commands::global_slash_commands;
use crate::context_menu_command::{ContextMenuCommandDetails, GetContextMenuCommandDetails};
use crate::global_slash_command::{GetSlashCommandDetails, GlobalSlashCommandDetails};
use serenity::prelude::GatewayIntents;

mod bot;
mod command_result;
mod commands;
mod context_menu_command;
mod global_slash_command;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = bot::get_token().await?;
    let intents = GatewayIntents::empty();

    bot::start(
        token,
        intents,
        SLASH_COMMANDS_LIST.clone(),
        CONTEXT_COMMANDS_LIST.clone(),
    )
    .await?;
    Ok(())
}

static SLASH_COMMANDS_LIST: once_cell::sync::Lazy<Vec<GlobalSlashCommandDetails>> =
    once_cell::sync::Lazy::new(|| {
        let commands = vec![
        global_slash_commands::cat_facts::CatFactsCommand::get_slash_command_details(),
        global_slash_commands::useless_facts::UselessFactsCommand::get_slash_command_details(),
        global_slash_commands::number_of_the_day::NumberOfTheDay::get_slash_command_details(),
        global_slash_commands::help::Help::get_slash_command_details(),
        global_slash_commands::api_ninjas_facts::ApiNinjasFacts::get_slash_command_details(),
        global_slash_commands::api_ninjas_trivia::ApiNinjasTrivia::get_slash_command_details(),
        global_slash_commands::api_ninja_whois_domain::WhoIsCommand::get_slash_command_details(),
        global_slash_commands::dad_jokes::DadJokesCommand::get_slash_command_details(),
        global_slash_commands::dictionary::DictionarySlashCommand::get_slash_command_details(),
        global_slash_commands::urban_dictionary::UrbanDictionaryCommand::get_slash_command_details(),
        global_slash_commands::hobby::HobbySlashCommand::get_slash_command_details(),
        global_slash_commands::text_language::TextLanguageSlashCommand::get_slash_command_details(),
        global_slash_commands::random_word::RandomWordSlashCommand::get_slash_command_details(),
        global_slash_commands::thesaurus::ThesaurusSlashCommand::get_slash_command_details(),
        global_slash_commands::riddle::RiddleSlashCommand::get_slash_command_details(),
    ];

        commands
    });

static CONTEXT_COMMANDS_LIST: once_cell::sync::Lazy<Vec<ContextMenuCommandDetails>> =
    once_cell::sync::Lazy::new(|| {
        let commands = vec![
        context_menu_commands::user_facts::UserFactsContextCommand::get_context_menu_command_details()
    ];

        commands
    });
