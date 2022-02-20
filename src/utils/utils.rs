use serenity::{
    http::Http,
    model::{id::ChannelId, interactions::application_command::ApplicationCommandInteraction},
};

use crate::{
    commands::setup::{interact, ResponseType},
    BotError,
};

// TODO maybe turn them into refernces
pub enum LogMedium {
    Slash(Box<ApplicationCommandInteraction>, ResponseType),
    Message(ChannelId),
}

#[allow(dead_code)]
struct Config<'a> {
    channel: &'a ChannelId,
    medium: &'a LogMedium,
}

// TODO add support for a logging channel
fn config(medium: &LogMedium) -> Result<Config, BotError> {
    match medium {
        LogMedium::Slash(interaction, _) => {
            let channel = &interaction.channel_id;

            Ok(Config { channel, medium })
        }
        LogMedium::Message(channel) => Ok(Config { channel, medium }),
    }
}

// fn delete_after(msg: Message) {}

pub async fn log_err(http: &Http, err: BotError, medium: LogMedium) {
    let config = config(&medium).expect("Fatal: Failed to get config");

    match medium {
        LogMedium::Slash(interaction, response_type) => {
            let content = err.to_string();
            interact(http, content, &interaction, response_type).await;
        }
        LogMedium::Message(_) => {
            let content = err.to_string();
            config
                .channel
                .say(http, content)
                .await
                .expect("Fatal: Failed to send message in the channel");
        }
    };
}
