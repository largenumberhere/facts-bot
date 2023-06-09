use crate::bot;
use crate::command_result::{CommandError, CommandSuccess, ToCommandResult};
use crate::global_slash_command::{GetSlashCommandDetails, GlobalSlashCommandDetails};
use futures::FutureExt;
use rand::Rng;
use serde::{Deserialize, Serialize};
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::Interaction;

pub struct CatFactsCommand {}

impl GetSlashCommandDetails for CatFactsCommand {
    fn get_slash_command_details() -> GlobalSlashCommandDetails {
        return GlobalSlashCommandDetails {
            name: "daily_cat_facts".to_string(),
            description: "Get one of 5 cat facts updated daily at from https://alexwohlbruck.github.io/cat-facts/".to_string(),
            options: Vec::new().into(),
            handler: |command_interaction, context, interaction| handler(command_interaction, context, interaction).boxed(),
            force_command_update: None,
        };
    }
}

async fn handler(
    _command_interaction: &ApplicationCommandInteraction,
    _context: &Context,
    _interaction: &Interaction,
) -> Result<CommandSuccess, CommandError> {
    //command_interaction.quick_reply(cat_info.to_string(),&context.http).await;

    // let response_json = {
    //     let url = "http://cat-fact.herokuapp.com/facts".parse().unwrap();
    //     let mut response = client.get(url).await.to_command_result()?;
    //
    //     let mut buff = Vec::new();
    //     while let Some(next) = response.body_mut().data().await {
    //         let chunk = next.to_command_result()?;
    //         //buff.append(&mut chunk.to_vec());
    //         buff.extend_from_slice(chunk.as_ref());
    //         // for byte in chunk{
    //         //     buff.push(byte);
    //         // }
    //     }
    //
    //
    //     String::from_utf8(buff).to_command_result()?
    // };

    //let response: CatFactsResponse = serde_json::from_str(response_json.as_str()).to_command_result()?;
    let response_json = bot::HttpClient::http_get_json(
        "http://cat-fact.herokuapp.com/facts"
            .parse()
            .to_command_result()?,
    )
    .await
    .to_command_result()?;
    let facts_response_data: Vec<CatFactData> =
        serde_json::from_str(response_json.as_str()).to_command_result()?;

    let mut random = rand::thread_rng();
    let fact_number = random.gen_range(0..facts_response_data.len());
    let text = &facts_response_data[fact_number].text;

    Ok(CommandSuccess::SuccessWithReply(text.to_string()))
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CatFactData {
    pub status: Status,
    #[serde(rename = "_id")]
    pub id: String,
    pub user: String,
    pub text: String,
    #[serde(rename = "__v")]
    pub v: i64,
    pub source: String,
    pub updated_at: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub created_at: String,
    pub deleted: bool,
    pub used: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    pub verified: bool,
    pub feedback: Option<String>,
    pub sent_count: i64,
}
