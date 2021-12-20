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
use std::io::{Read, Write, Error};


#[group]
#[commands(ping)]
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


#[tokio::main]
async fn main() {
    let c = load_cfg().expect("Error loading config file.");

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("/"))
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(c.token)
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
    msg.reply(&ctx, "Pong!").await?;

    Ok(())
}
