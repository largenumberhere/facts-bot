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
    let commands:Vec<GlobalSlashCommandDetails<_>> = vec![
        commands::cat_facts::CatFactsCommand::get_command_details()
    ];

    bot::start(bot::get_token().await, GatewayIntents::empty(), commands).await.unwrap();
}

// static TOKIO_RUNTIME: once_cell::sync::Lazy<tokio::runtime::Runtime> = once_cell::sync::Lazy::new(start_tokio);
// fn start_tokio() -> tokio::runtime::Runtime{
//     let runtime = tokio::runtime::Builder::new_multi_thread()
//         .worker_threads(1)
//         .enable_all()
//         .build()
//         .unwrap();
//     runtime
// }


// pub struct Task<Input,Output>{
//     func: fn(input: Input) -> Future<Output = Output>
// }
//
// pub struct TaskSpawner{
//     spawn: tokio::sync::mpsc::Sender<Task>
// }
//
// impl TaskSpawner{
//     pub fn new()->{
//         let (send, mut recv) = tokio::sync::mpsc::channel(16);
//
//         let runtime = tokio::runtime::Builder::new_current_thread()
//             .enable_all()
//             .build()
//             .unwrap();
//
//         std::thread::spawn(move || {
//             runtime.block_on(async move ||{
//                 while let Some(task) = recv.recv().await{
//                     tokio::spawn(handle_task);
//                 }
//             })
//         })
//     }
// }