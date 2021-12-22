#[macro_use]
extern crate lazy_static;

use serenity::{
    async_trait,
    client::{Client, Context, EventHandler},
    model::channel::Message,
    framework::standard::{
        StandardFramework,
        CommandResult,
        macros::{command, group}
    }
};
use std::fs::File;
use std::io::{Read, Write, Error, Split};
use std::sync::Mutex;
use rand::prelude::*;


struct Row (Vec<String>);
struct Table (Vec<Row>);
impl Table {
    fn parse(csv: &str) -> Table {
        let mut table: Vec<Row> = Vec::new();
        let rows: Vec<String> = csv.to_string().split("\n").collect::<Vec<String>>();
        for r in rows {
            let row: Row = Row(r.split(",").collect::<Vec<String>>());
            table.push(row);
        }
        Table(table)
    }
}


#[group]
#[commands(ping, roll)]
struct General;

struct Handler;
#[async_trait]
impl EventHandler for Handler {}


struct Cfg {
    token: String,
    adjust_rolls: bool
}

fn load_cfg() -> Result<Cfg, Error> {
    let mut f = File::open("config");
    if let Err(why) = f {
        let data_string = "TOKEN\nfalse";
        let mut n = File::create("config")?;
        n.write_all(data_string.as_bytes())?;
        Ok(Cfg {
            token: "TOKEN".to_string(),
            adjust_rolls: false
        })
    } else {
        let mut data_string = String::new();
        f?.read_to_string(&mut data_string);
        let data = data_string.split("\n").collect::<Vec<&str>>();
        let ar = data[1] == "true";
        let token = data[0].to_string();
        Ok(
            Cfg {
                token: token,
                adjust_rolls: ar
            }
        )
    }
}


lazy_static! {
    static ref CFG: Cfg = load_cfg().unwrap();
}


#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("/"))
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(&CFG.token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client.");

    if let Err(why) = client.start().await {
        println!("Error: {:?}", why);
    }
}


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
    let adjust = match &CFG.adjust_rolls {
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
