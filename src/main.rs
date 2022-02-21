use bot::commands::setup::slash_react;
use bot::commands::setup::slash_setup;
use bot::commands::verify::verify;
use bot::commands::verify::Details;
use serenity::async_trait;
use serenity::client::{Client, EventHandler};
use serenity::framework::standard::{macros::group, StandardFramework};
use serenity::model::channel::Message;
use serenity::model::prelude::Interaction;
use serenity::model::prelude::Reaction;
use serenity::model::prelude::Ready;
use serenity::prelude::Context;

#[group]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, _ctx: Context, _msg: Message) {}

    async fn reaction_add(&self, ctx: Context, _add_reaction: Reaction) {
        let details = Details {
            message_id: 888108775536468060,
            channel_id: 888108235985391657,
            role_name: "peeps",
        };

        verify(&ctx, details).await.unwrap();
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        slash_setup(&ctx.http).await.unwrap();
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            slash_react(&ctx.http, &command).await;
        }
    }
}

// TODO research about format strings and fix them
// TODO add binary distribute command
#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("%")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = std::env::var("DISCORD_TOKEN").expect("token");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .application_id(883958088594489394)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
