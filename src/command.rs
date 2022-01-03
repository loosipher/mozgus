use std::sync::Arc;
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
use crate::cache::CACHE;


#[group]
#[commands(ping, roll, stat)]
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
    let skill = &msg.content.split_whitespace().collect::<Vec<&str>>()[1..].join(" ");
    let mut success = false;

    let value = Arc::clone(&CACHE).lock().unwrap().get_skill(&usn, skill.as_str());
    if value.is_none() {
        msg.reply(&ctx, format!("Can't find the {} skill for {}", skill, usn)).await?;
        return Ok(());
    }
    success = total <= value.unwrap();

    let goal = if success {
        "SUCCESS"
    } else {
        "FAIL"
    };

    let mtext = format!("{} rolled for {} ({} - {} = {}) {}", usn, skill, r, adjust, total, goal);

    msg.reply(&ctx, mtext).await?;

    Ok(())
}

#[command]
async fn stat(ctx: &Context, msg: &Message) -> CommandResult {
    // syntax: /stat [STAT]
    let usn = match msg.author_nick(&ctx).await {
        Some(nick) => nick,
        _ => msg.author.name.to_string()
    };
    let stat = &msg.content.split_whitespace().collect::<Vec<&str>>()[1..].join(" ");
    let value = CACHE.lock().unwrap().get_stat(&usn, stat);
    if value.is_none() {
        msg.reply(&ctx, format!("{} not found for {}", stat, usn)).await?;
    }

    let mtext = format!("{}'s {} stat is {}", usn, stat, value.unwrap());

    msg.reply(&ctx, mtext).await?;

    Ok(())
}
