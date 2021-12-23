use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::channel::Message,
    framework::standard::{
        CommandResult,
        macros::{command, group}
    }
};
use rand::prelude::*;


#[group]
#[commands(ping, roll)]
struct General;

pub struct Handler;
#[async_trait]
impl EventHandler for Handler {}


#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    // syntax: /ping
    msg.reply(&ctx, "Pong!").await?;

    Ok(())
}

#[command]
async fn roll(ctx: &Context, msg: &Message) -> CommandResult {
    // syntax: /roll [SKILL]
    let r = rand::thread_rng().gen::<u32>() % 100;
    let adjust = match &crate::config::CFG.adjust_rolls {
        true => 5,
        false => 0
    } as i32;
    let total = (r as i32) - adjust;
    let usn = match msg.author_nick(&ctx).await {
        Some(nick) => nick,
        _ => msg.author.name.to_string()
    }.to_string();
    let skill = *(&msg.content.split(" ").collect::<Vec<&str>>()[1]);

    let mtext = format!("{} rolled for {} ({} - {} = {}) {}", usn, skill, r, adjust, total, "SUCCESS");

    msg.reply(&ctx, mtext).await?;

    Ok(())
}
