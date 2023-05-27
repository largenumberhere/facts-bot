use std::future::Future;
use std::ops::Deref;
use futures::{FutureExt, TryFutureExt};
use hyper::body::HttpBody;
use hyper::Uri;
use serde_json::Value;
use serenity::async_trait;
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::Interaction;
use crate::bot::QuickReply;
use crate::global_slash_command::{CommandError, CommandSuccess, GetCommandDetails, GlobalSlashCommandDetails, ToCommandResult};
use serde::{Deserialize, Serialize};
use rand::Rng;
pub struct CatFactsCommand{}

impl GetCommandDetails for CatFactsCommand {
    fn get_command_details() -> GlobalSlashCommandDetails
    {
        return GlobalSlashCommandDetails {
            name: "daily_cat_facts".to_string(),
            description: "Get one of 5 cat facts updated daily at from https://alexwohlbruck.github.io/cat-facts/".to_string(),
            options: Vec::new().into(),
            handler: |command_interaction, context, interaction| handler(command_interaction, context, interaction).boxed()
        }
    }
}

async fn handler(command_interaction: &ApplicationCommandInteraction, context: &Context, interaction: &Interaction) -> Result<CommandSuccess, CommandError>{
    let cat_info = "cats!";

    //command_interaction.quick_reply(cat_info.to_string(),&context.http).await;
    let client = hyper::client::Client::new();

    let response_json = {
        let url = "http://cat-fact.herokuapp.com/facts".parse().unwrap();
        let mut response = client.get(url).await.to_command_result()?;

        let mut buff = Vec::new();
        while let Some(next) = response.body_mut().data().await {
            let chunk = next.to_command_result()?;
            //buff.append(&mut chunk.to_vec());
            buff.extend_from_slice(chunk.as_ref());
            // for byte in chunk{
            //     buff.push(byte);
            // }
        }


        String::from_utf8(buff).to_command_result()?
    };


    //let response: CatFactsResponse = serde_json::from_str(response_json.as_str()).to_command_result()?;

    let facts_response_data: Vec<CatFactData>= serde_json::from_str(response_json.as_str()).to_command_result()?;

    let mut random = rand::thread_rng();
    let fact_number = random.gen_range(0..facts_response_data.len());
    let text = &facts_response_data[fact_number].text;


    Ok(CommandSuccess::SuccessWithReply(text.to_string())) }


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