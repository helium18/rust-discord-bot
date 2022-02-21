use self::clear::{command_clear, command_clear_setup};
use super::memes::{command_meme, command_memes_setup};
use super::slashembed::{command_embed, command_embed_setup};
use crate::utils::log_err;
use crate::utils::LogMedium;
use crate::BotError;
use async_recursion::async_recursion;
use serenity::builder::CreateApplicationCommandOption;
use serenity::builder::CreateInteractionResponseData;
use serenity::http::Http;
use serenity::model::channel::Channel;
use serenity::model::interactions::application_command::{
    ApplicationCommand, ApplicationCommandInteraction,
};
use serenity::model::interactions::{
    InteractionApplicationCommandCallbackDataFlags, InteractionResponseType,
};

mod clear;

pub enum ResponseType {
    Normal,
    Ephemeral,
}

pub async fn slash_setup(http: &Http) -> Result<(), BotError> {
    global_slash("father", "Summon the father", vec![], http).await?;
    global_slash("cock", "cock ?!?!?!?!", vec![], http).await?;
    command_clear_setup(http).await?;
    command_memes_setup(http).await?;
    command_embed_setup(http).await?;
    Ok(())
}

pub async fn slash_react(http: &Http, command: &ApplicationCommandInteraction) {
    let mut guild_channel = None;
    let mut content = CreateInteractionResponseData::default();

    if let Channel::Guild(channel) = command.channel_id.to_channel(http).await.expect("channel") {
        guild_channel = Some(channel);
    }

    match command.data.name.as_str() {
        "cock" => {
            content
                .content("I wanna sex so bad <:bite:938339213382942771>")
                .flags(InteractionApplicationCommandCallbackDataFlags::empty());

            interact(http, content, command).await
        }
        "father" => {
            content
                .content("Father <@!710342832145039390> <:bite:938339213382942771>")
                .flags(InteractionApplicationCommandCallbackDataFlags::empty());
            interact(http, content, command).await
        }
        "clear" => {
            match command_clear(command, http, guild_channel.expect("Empty guild channel")).await {
                Ok(v) | Err(v) => {
                    content
                        .content(v)
                        .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL);
                    interact(http, content, command).await;
                }
            }
        }
        // TODO Fix `.unwrap()` and use `log_err()` instead
        "embed" => {
            match command_embed(command, http, guild_channel.expect("Empty guild channel")).await {
                Ok(_) => {
                    content
                        .content("The embed was sent!")
                        .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL);
                    interact(http, content, command).await;
                }
                Err(err) => {
                    content
                        .content(err)
                        .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL);
                    interact(http, content, command).await;
                }
            }
        }
        // TODO Errors should tell you to report the error the me
        "memes" => match command_meme(command).await {
            Ok(v) => {
                interact(http, v, command).await;
            }
            Err(err) => {
                log_err(
                    http,
                    err,
                    LogMedium::Slash(Box::new(command.to_owned()), ResponseType::Ephemeral),
                )
                .await
            }
        },
        _ => {
            content
                .content("Imagine demanding for non-existent commands lol")
                .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL);

            interact(http, content, command).await
        }
    };
}

pub async fn global_slash(
    name: &str,
    description: &str,
    options: Vec<CreateApplicationCommandOption>,
    http: &Http,
) -> Result<(), BotError> {
    let commands = ApplicationCommand::create_global_application_command(http, |command| {
        match options.len() {
            0 => command.name(name).description(description),
            _ => {
                command.name(name).description(description);
                for opt in options.into_iter() {
                    command.add_option(opt);
                }
                command
            }
        }
    })
    .await
    .map_err(|err| BotError::GlobalSlashCommandsCreationFail(format!("{:#?}", err)))?;

    println!("The command `{}` was registered", commands.name);

    Ok(())
}

#[async_recursion]
pub async fn interact(
    http: &Http,
    content: CreateInteractionResponseData,
    interaction: &ApplicationCommandInteraction,
) {
    if let Err(why) = {
        interaction
            .create_interaction_response(http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|c| {
                        c.clone_from(&content);
                        c
                    })
            })
            .await
    } {
        let error = format!("Failed slash command interaction.\nReason: {}", why);
        println!("{}", &error);
        log_err(
            http,
            BotError::SlashCommandInteractionFail(why.to_string()),
            LogMedium::Message(interaction.channel_id),
        )
        .await;
    }
}
