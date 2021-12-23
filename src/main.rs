 #[macro_use]
extern crate lazy_static;

use serenity::{
    client::Client,
    framework::standard::StandardFramework
};

mod config;
mod command;
mod csreader;


#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("/"))
        .group(&crate::command::GENERAL_GROUP);

    let mut client = Client::builder(&config::CFG.token)
        .event_handler(command::Handler)
        .framework(framework)
        .await
        .expect("Error creating client.");

    if let Err(why) = client.start().await {
        println!("Error: {:?}", why);
    }
}
