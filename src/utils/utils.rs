use std::{thread, time::Duration};

use serenity::{
    builder::CreateInteractionResponseData,
    http::Http,
    model::{
        channel::Message,
        id::ChannelId,
        interactions::{
            application_command::ApplicationCommandInteraction,
            InteractionApplicationCommandCallbackDataFlags,
        },
    },
};

use crate::{
    commands::setup::{interact, ResponseType},
    BotError, COLOR, FOOTER_IMAGE,
};

pub enum LogMedium {
    Slash(Box<ApplicationCommandInteraction>, ResponseType),
    Message(ChannelId),
}

// TODO add support for a logging channel
fn channel_id(medium: &LogMedium) -> Result<&ChannelId, BotError> {
    match medium {
        LogMedium::Slash(interaction, _) => {
            let channel = &interaction.channel_id;

            Ok(channel)
        }
        LogMedium::Message(channel) => Ok(channel),
    }
}

async fn delete_after(http: &Http, dur: Duration, msg: Message) -> Result<(), BotError> {
    thread::sleep(dur);
    msg.delete(http)
        .await
        .map_err(|err| BotError::DeleteMessageFail(err.to_string()))?;

    Ok(())
}

pub async fn log_err(http: &Http, err: BotError, medium: LogMedium) {
    let channel_id = channel_id(&medium).expect("Fatal: Failed to get config");

    match medium {
        LogMedium::Slash(interaction, _) => {
            let mut content = CreateInteractionResponseData::default();
            content
                .create_embed(|embed| {
                    embed
                        .title("No Damsels ?! \\😧")
                        .description(err)
                        .color(COLOR)
                        .footer(|footer| footer.icon_url(FOOTER_IMAGE))
                })
                .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL);
            interact(http, content, &interaction).await;
        }
        LogMedium::Message(_) => {
            let content = err.to_string();
            let message = channel_id
                .send_message(http, |msg| {
                    msg.embed(|embed| {
                        embed
                            .title("No Damsels ?! \\😧")
                            .description(content)
                            .color(COLOR)
                            .footer(|footer| footer.icon_url(FOOTER_IMAGE)) // Footer doesn't work
                    })
                })
                .await
                .expect("Fatal: Failed to create message");
            delete_after(http, Duration::from_secs(10), message)
                .await
                .expect("Fatal: Failed to delete error message");
        }
    };
}
