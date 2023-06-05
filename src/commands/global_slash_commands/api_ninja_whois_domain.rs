use std::ptr::write;
use std::str::FromStr;
use futures::FutureExt;
// use hyper::http::uri::InvalidUri;
// use hyper::Uri;
use itertools::Itertools;
use serde_json::{json, Value};
use serenity::builder::CreateApplicationCommandOption;
use serenity::client::Context;
use serenity::model::application::command::CommandOptionType;
use serenity::model::application::interaction::application_command::{ApplicationCommandInteraction, CommandDataOptionValue};
use serenity::model::application::interaction::Interaction;
use serenity::model::prelude::command::CommandOption;
use crate::bot;
use crate::command_result::{CommandError, CommandSuccess, ToCommandResult};
use crate::command_result::CommandError::InternalError;
use crate::global_slash_command::{GetSlashCommandDetails, GlobalSlashCommandDetails};
use serde::{Serialize, Deserialize, Deserializer, de};
use std::fmt::{Formatter, Write};
use std::marker::PhantomData;
use serde::de::Error;

pub struct WhoIsCommand{}

impl GetSlashCommandDetails for WhoIsCommand{
    fn get_slash_command_details() -> GlobalSlashCommandDetails {
        GlobalSlashCommandDetails{
            name: "who_is_domain".to_string(),
            description: "Get domain registration information for a given domain using https://api-ninjas.com/api/whois".to_string(),
            options: vec![
                CreateApplicationCommandOption::default()
                    .name("domain")
                    .kind(CommandOptionType::String)
                    .required(true)
                    .description("Pick a domain to get the information about").to_owned()
            ],
            handler: |command_interaction, context, interaction| handler(command_interaction, context, interaction).boxed(),
            force_command_update: None
        }
    }
}

async fn handler (_command_interaction: &ApplicationCommandInteraction, _context: &Context, _interaction: &Interaction) -> Result<CommandSuccess, CommandError>{
    let ninja_facts_key = bot::get_token_from("api-ninjas-com-key.file".to_string()).await.to_command_result()?;

    let option1 =_command_interaction.data.options.first();
    let option1 = match option1 {
        Some(v)=>{
            v
        },
        None=>{
            return Err(CommandError::InvalidUserInputError("There must be one domain to fetch info for.".to_string()))
        }
    };

    let domain_string = match &option1.resolved {
        None => {
            return Err(CommandError::InvalidUserInputError("There must be one string with a url to fetch domain formation about. ".to_string()))
        }
        Some(v) => {
            match v {
                CommandDataOptionValue::String(v) => {
                    v
                }
                _=>{
                    return Err(CommandError::InvalidUserInputError("Invalid type received. The domain must be a string".to_string()))
                }
            }
        }
    };

    //validate the uri
    // let uri =  reqwest::Url::from_str(domain_string);
    // let uri = match uri {
    //     Ok(v) => {
    //         v
    //     }
    //     Err(e) => {
    //         return Err(CommandError::InvalidUserInputError(format!("Not a domain: '{:?}'",e)))
    //     }
    // };

    let mut base_url = "https://api.api-ninjas.com/v1/whois?domain=".to_string();
    base_url.push_str(domain_string);


    let uri: reqwest::Url = base_url.parse().to_command_result()?;
    let json = bot::HttpClient::https_get_json_with_headers(uri, vec![("X-Api-Key",ninja_facts_key.as_str())]).await.to_command_result()?;

    if json == "{}" {
        return Ok(CommandSuccess::SuccessWithReply("Empty response received. Are you sure you sent a registered domain?".to_string()))
    }


    let response: Result<DomainDetailsResponse, _> = serde_json::from_str(json.as_str());

    let response: DomainDetails = match response {
        Err(e) =>{
            return Err(CommandError::InternalError(format!("Json parse error: '{:?}'\n value: {:#?}",json,e)))
        },
        Ok(v) => v
    }.into();

    let reply_string = {
        let mut reply_string = String::new();

        writeln!(&mut reply_string, "**Results for `{}`**", response.domain_name.first().ok_or(CommandError::InternalError("Failed to read first url".to_string()))?).to_command_result()?;
        writeln!(&mut reply_string, "Registrar: {}", response.registrar).to_command_result()?;
        writeln!(&mut reply_string, "Whois server: {}", response.whois_server).to_command_result()?;

        write!(&mut reply_string, "Creation date(s): ").to_command_result()?;
        for date in response.creation_date.into_iter() {
            write!(&mut reply_string, "<t:{}:d>, ", date).to_command_result()?;
        }
        write!(&mut reply_string, "\n").to_command_result()?;

        write!(&mut reply_string, "Expiration date(s): ").to_command_result()?;
        for date in response.expiration_date.into_iter() {
            write!(&mut reply_string, "<t:{}:d>, ", date).to_command_result()?;
        }
        write!(&mut reply_string, "\n").to_command_result()?;

        write!(&mut reply_string, "Name servers: ").to_command_result()?;
        for server in response.name_servers.into_iter() {
            write!(&mut reply_string, "{}, ", server).to_command_result()?;
        }
        write!(&mut reply_string, "\n").to_command_result()?;

        write!(&mut reply_string, "Email(s): ").to_command_result()?;
        for email in response.emails.into_iter() {
            write!(&mut reply_string, "{}, ", email).to_command_result()?;
        }
        write!(&mut reply_string, "\n").to_command_result()?;

        writeln!(&mut reply_string, "Domain name security extensions: {}", response.dnssec).to_command_result()?;
        writeln!(&mut reply_string, "Org: {}", response.org).to_command_result()?;
        writeln!(&mut reply_string, "State: {}", response.state).to_command_result()?;
        writeln!(&mut reply_string, "Country: {}", response.country).to_command_result()?;

        reply_string
    };

    Ok(CommandSuccess::SuccessWithReply(reply_string))
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum Int64OrVecInt{
    Int(i64),
    Vec(Vec<i64>)
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum StringOrVecString{
    String(String),
    Vec(Vec<String>)
}

#[derive(Serialize, Deserialize)]
struct DomainDetailsResponse {
    pub domain_name: StringOrVecString,
    pub registrar: String,
    pub whois_server: String,
    pub updated_date: Int64OrVecInt,
    pub creation_date: Int64OrVecInt,
    pub expiration_date: Int64OrVecInt,
    pub name_servers: StringOrVecString,
    pub emails: StringOrVecString,
    pub dnssec: String,
    pub org: String,
    pub state: String,
    pub country: String,
}

struct DomainDetails{
    pub domain_name: Vec<String>,
    pub registrar: String,
    pub whois_server: String,
    pub updated_date: Vec<i64>,
    pub creation_date: Vec<i64>,
    pub expiration_date: Vec<i64>,
    pub name_servers: Vec<String>,
    pub emails: Vec<String>,
    pub dnssec: String,
    pub org: String,
    pub state: String,
    pub country: String,
}

impl From<DomainDetailsResponse> for DomainDetails {
    fn from(value: DomainDetailsResponse) -> DomainDetails {
        DomainDetails{
            domain_name: match value.domain_name {
                StringOrVecString::String(v) => {vec![v]}
                StringOrVecString::Vec(v) => {v}
            },
            registrar: value.registrar,
            whois_server: value.whois_server,
            updated_date: {
                match value.updated_date {
                    Int64OrVecInt::Int(v) => {vec![v]}
                    Int64OrVecInt::Vec(v) => {v}
                }
            },
            creation_date: {
                match value.creation_date {
                    Int64OrVecInt::Int(v) => {vec![v]}
                    Int64OrVecInt::Vec(v) => {v}
                }
            },
            expiration_date: {
                match value.expiration_date {
                    Int64OrVecInt::Int(v) => {vec![v]}
                    Int64OrVecInt::Vec(v) => {v}
                }
            },
            name_servers: match value.name_servers{
                StringOrVecString::String(v) => {vec![v]}
                StringOrVecString::Vec(v) => { v}
            },
            emails: match value.emails {
                StringOrVecString::String(v) => {vec![v]}
                StringOrVecString::Vec(v) => {v}
            },
            dnssec: value.dnssec,
            org: value.org,
            state: value.state,
            country: value.country
        }
    }
}
