use serenity::prelude::GatewayIntents;
use crate::global_slash_command::{GlobalSlashCommand};

mod commands;
mod global_slash_command;
mod bot;

#[tokio::main]
async fn main() {
    let commands: Vec<&dyn GlobalSlashCommand> = vec![
        commands::cat_facts::CatFactsCommand{} as &dyn GlobalSlashCommand
    ];

    bot::start(bot::get_token().await, GatewayIntents::empty(), commands).await.unwrap();
}
