use std::fmt::{Debug, Error};
use std::future::Future;
use serenity::prelude::GatewayIntents;
use crate::global_slash_command::{GetCommandDetails, GlobalSlashCommandDetails};

mod commands;
mod global_slash_command;
mod bot;

#[tokio::main]
async fn main() -> Result<(), Box<dyn  std::error::Error>>  {

    let token = bot::get_token().await?;
    let intents = GatewayIntents::empty();

    bot::start(token, intents, COMMANDS_LIST.clone()).await?;
    Ok(())
}

static COMMANDS_LIST: once_cell::sync::Lazy<Vec<GlobalSlashCommandDetails>> = once_cell::sync::Lazy::new(||{
    let commands = vec![
        commands::cat_facts::CatFactsCommand::get_command_details(),
        commands::useless_facts::UselessFactsCommand::get_command_details(),
        commands::number_of_the_day::NumberOfTheDay::get_command_details(),
        commands::help::Help::get_command_details(),
        commands::api_ninjas_facts::ApiNinjasFacts::get_command_details()
    ];

    commands
});
