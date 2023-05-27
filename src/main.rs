use std::future::Future;
use serenity::prelude::GatewayIntents;
use crate::global_slash_command::{GetCommandDetails, GlobalSlashCommandDetails};

mod commands;
mod global_slash_command;
mod bot;

// fn main() {
//     TOKIO_RUNTIME.block_on(
//         main_async()
//     );
// }
#[tokio::main]
async fn main(){
    let commands = vec![
        commands::cat_facts::CatFactsCommand::get_command_details()
    ];

    let token = bot::get_token().await;
    let intents = GatewayIntents::empty();

    bot::start(token, intents, commands).await.unwrap();
}

// static HTTP_CLIENT: once_cell::sync::Lazy<hyper::Client<hyper::client::HttpConnector>>
// = once_cell::sync::Lazy::new(||{
//     hyper::Client::new()
// });