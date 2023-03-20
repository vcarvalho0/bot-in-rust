use std::env;

use serenity::{
    async_trait,
    framework::{standard::{
        CommandOptions, CommandGroup, CommandResult,
        macros::{command, group}
    }, StandardFramework},
    model::{channel::Message, gateway::Ready},
    prelude::*
};

#[group]
#[commands(hello)]
struct General;

#[command]
async fn hello(ctx: &Context, msg: &Message) -> CommandResult {
    if let Err(why) = msg.channel_id.say(&ctx.http, "Hello World!").await {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    let framework = StandardFramework::new()
    .configure(|config_obj| config_obj.prefix("."));

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client =
        Client::builder(&token, intents)
            .event_handler(Handler)
            .framework(framework)
            .await
            .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}