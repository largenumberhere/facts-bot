use serenity::prelude::GatewayIntents;
use crate::global_slash_command::{GetCommandDetails, GlobalSlashCommandDetails};

mod commands;
mod global_slash_command;
mod bot;

#[tokio::main]
async fn main() {
    let commands: Vec<GlobalSlashCommandDetails> = vec![
        commands::cat_facts::CatFactsCommand::get_command_details()
    ];

    bot::start(bot::get_token().await, GatewayIntents::empty(), commands).await.unwrap();
}
