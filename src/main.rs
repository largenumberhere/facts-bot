use serenity::client::{Context, EventHandler};
use serenity::model::application::interaction::Interaction;
use serenity::model::application::interaction::InteractionResponseType::ChannelMessageWithSource;
use serenity::model::gateway::Ready;
use serenity::model::prelude::command::Command;
use serenity::prelude::GatewayIntents;

mod commands;
mod global_slash_command;
mod bot;

#[tokio::main]
async fn main() {
    let commands = vec![



    ];

    bot::start(bot::get_token(), GatewayIntents::empty(), commands).await.unwrap();
}
