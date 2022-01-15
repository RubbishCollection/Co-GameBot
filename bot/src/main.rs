mod error;
mod game;
mod game_manager;

pub use crate::error::*;
pub use crate::game::*;
pub use crate::game_manager::*;

use crate::dummy::Dummy;

use serenity::async_trait;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::CommandResult;
use serenity::framework::StandardFramework;
use serenity::model::channel::Message;
use serenity::model::interactions::Interaction;
use serenity::prelude::*;

use std::sync::Arc;
use std::time::Duration;

use parking_lot::Mutex;

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Help your self").await?;

    Ok(())
}

#[group]
#[commands(help)]
struct General;

struct MessageHandler {
    gm: Mutex<GameManager>,
}

impl MessageHandler {
    fn init(gm: GameManager) -> Self {
        Self { gm: Mutex::new(gm) }
    }
}

#[async_trait]
impl EventHandler for MessageHandler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "$startd" {
            let dg = Box::new(Dummy::new());

            let mut gm = self.gm.lock();
            gm.register(ctx, msg.channel_id, dg, Duration::from_secs(1000))
                .await;
        }
    }
    async fn interaction_create(&self, ctx: Context, interact: Interaction) {
        match interact {
            Interaction::Ping(_) => return,
            Interaction::ApplicationCommand(_) => return,
            Interaction::MessageComponent(interact) => {
                let mut gm = self.gm.lock();
                gm.handle_interaction(ctx, interact).await;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let gm = GameManager::new();

    let token = std::env::var("DISCORD_TOKEN").expect("token not exist");

    let application_id: u64 = std::env::var("APPLICATION_ID")
        .expect("application id not exist")
        .parse()
        .expect("app_id parse failed");

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("$"))
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(&token)
        .event_handler(MessageHandler::init(gm))
        .framework(framework)
        .application_id(application_id)
        .await
        .expect("client creation failed");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
