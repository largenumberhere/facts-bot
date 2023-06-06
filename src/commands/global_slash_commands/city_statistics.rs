// too much work to implement

// use serenity::client::Context;
// use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
// use serenity::model::application::interaction::Interaction;
// use crate::command_result::{CommandError, CommandSuccess, ToCommandResult, ToCommandResultWith};
// use crate::global_slash_command::{GetSlashCommandDetails, GlobalSlashCommandDetails};
// use futures::FutureExt;
// use crate::bot;
//
// struct CityStatisticsCommand{}
// impl GetSlashCommandDetails for CityStatisticsCommand{
//     fn get_slash_command_details() -> GlobalSlashCommandDetails {
//         GlobalSlashCommandDetails{
//             name: "city_statistics".to_string(),
//             description: "Get the statistics such as population for a random city from https://api-ninjas.com/api/city".to_string(),
//             options: vec![],
//             handler: |a, b, c|  handler(a,b,c).boxed(),
//             force_command_update: None
//         }
//     }
// }
//
// async fn handler(_command_interaction: &ApplicationCommandInteraction, _context: &Context, _interaction: &Interaction) -> Result<CommandSuccess, CommandError> {
//     let ninja_facts_key = bot::get_token_from("api-ninjas-com-key.file".to_string()).await.to_command_result_with("failed to read api key from file on server")?;
//
//     let uri = "https://api.api-ninjas.com/v1/city".parse().to_command_result_with("failed to parse city url")?;
//     let reply = bot::HttpClient::https_get_json_with_headers(uri,vec![("X-Api-Key", ninja_facts_key.as_str())]).await.to_command_result_with("failed to fetch reply from city server")?;
//
//
//     Ok(CommandSuccess::Success)
// }