use chrono::Utc;
use handlebars::Handlebars;
use serde_json::json;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::env;

static STANDUP_MESSAGE: &str = "
-------------------------------------
:ok_hand:            {{date}}              :ok_hand:
-------------------------------------
";

static HELP_MESSAGE: &str = "
try:
!help
!standup
";

const STANDUP_COMMAND: &str = "!standup";
const HELP_COMMAND: &str = "!help";

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == STANDUP_COMMAND {
            let date_string = Utc::now().format("%d.%m.%Y").to_string();

            let reg = Handlebars::new();
            let msg_string = reg
                .render_template(STANDUP_MESSAGE, &json!({"date": date_string.as_str()}))
                .unwrap();
            if let Err(why) = msg.channel_id.say(&ctx.http, msg_string).await {
                println!("Error sending message: {:?}", why);
            }

            msg.delete(ctx).await.unwrap();
        } else if msg.content == HELP_COMMAND {
            if let Err(why) = msg.channel_id.say(&ctx.http, HELP_MESSAGE).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}
#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::new(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
