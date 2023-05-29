use futures::FutureExt;
use hyper::body::HttpBody;
use hyper::{Body, HeaderMap, Method, Request, Uri};
use hyper::http::HeaderValue;
use reqwest::RequestBuilder;
use serde::{Serialize,Deserialize};
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::Interaction;
use crate::bot;
use crate::global_slash_command::{CommandError, CommandSuccess, GetCommandDetails, GlobalSlashCommandDetails, ToCommandResult};
use std::string::String;

pub struct ApiNinjasFacts{}

impl GetCommandDetails for ApiNinjasFacts{
    fn get_command_details() -> GlobalSlashCommandDetails {
        GlobalSlashCommandDetails{
            name: "facts".to_string(),
            description: "Get a random fact from https://api-ninjas.com/api/facts".to_string(),
            options: vec![],
            handler: |command_interaction, context, interaction| handler(command_interaction, context, interaction).boxed()
        }
    }
}

async fn handler(command_interaction: &ApplicationCommandInteraction, context: &Context, interaction: &Interaction) -> Result<CommandSuccess, CommandError> {
    let uri: Uri = "https://api.api-ninjas.com/v1/facts?limit=1".to_string().parse().to_command_result()?;
    //let json = bot::HttpClient::https_get_json(uri).await.to_command_result()?;
    // let json:&str = {
    //
    //
    //
    // }

    let https = hyper_tls::HttpsConnector::new();
    let client = hyper::client::Client::builder().build::<_, hyper::Body>(https);
    let ninja_facts_key = bot::get_token_from("api-ninjas-com-key.file".to_string()).await;

    /*let req = Request::builder()
    .method(Method::POST)
    .uri("http://httpbin.org/post")
    .body(Body::from("Hallo!"))
    .expect("request builder");
    */
    let req = Request::builder()
        .method(Method::GET)
        .header("X-Api-Key",ninja_facts_key)
        .uri("https://api.api-ninjas.com/v1/facts?limit=1")
        .body(Body::from(""))
        .to_command_result()?;

    let mut response =  client.request(req).await.to_command_result()?;

    //let mut connection = client.get(uri).await?;

    let mut buffer = Vec::new();

    while let Some(next) = response.body_mut().data().await {
        let chunk = next.to_command_result()?;
        buffer.extend_from_slice(chunk.as_ref());
    }
    let json = String::from_utf8(buffer).to_command_result()?;



    let facts: Vec<FactReply> = serde_json::from_str(json.as_str()).to_command_result()?;
    let fact = facts.get(0).ok_or_else(||"Failure to get fact 0".to_string()).to_command_result()?;

    Ok(CommandSuccess::SuccessWithReply(fact.fact.to_string()))
}


pub type FactsReply = Vec<FactReply>;
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FactReply {
    pub fact: String,
}

