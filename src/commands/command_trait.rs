use crate::global_slash_command::GlobalSlashCommand;

pub trait GetCommand{
    fn get_command() -> GlobalSlashCommand;
}