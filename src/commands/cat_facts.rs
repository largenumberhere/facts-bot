use serenity::http::CacheHttp;
use crate::bot::QuickReply;
use crate::commands::command_trait;
use crate::global_slash_command::GlobalSlashCommand;

struct CatFactsCommand{}
impl command_trait::GetCommand for CatFactsCommand{
    fn get_command() -> GlobalSlashCommand {
        return GlobalSlashCommand{
            name: "cat_facts".to_string(),
            description: "Get daily cat facts from https://cat-fact.herokuapp.com. Project website is at https://alexwohlbruck.github.io/cat-facts/".to_string(),
            options: Vec::new(),
            request_handler: |command_interaction, context, interaction|{


                let cat_info = "cats!";
                command_interaction.quick_reply(cat_info.to_string(),&context.http);

                Ok(())
            },



        }
    }
}

