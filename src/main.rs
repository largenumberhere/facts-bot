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

    let token = bot::get_token().await;
    let intents = GatewayIntents::empty();

    bot::start(token, intents, COMMANDS_LIST.clone()).await.unwrap();
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


// static HTTP_CLIENT: once_cell::sync::Lazy<hyper::Client<hyper::client::HttpConnector>>
// = once_cell::sync::Lazy::new(||{
//     hyper::Client::new()
// });