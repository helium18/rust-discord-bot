use serenity::builder::CreateApplicationCommandOption;
use serenity::http::Http;
use serenity::model::channel::Channel;
use serenity::model::interactions::application_command::{
    ApplicationCommand, ApplicationCommandInteraction,
};
use serenity::model::interactions::{
    InteractionApplicationCommandCallbackDataFlags, InteractionResponseType,
};
use std::error::Error;
use std::fmt::Display;

use self::clear::{command_clear, command_clear_setup};

use super::memes::{command_meme, command_memes_setup};

mod clear;

pub enum ResponseType {
    Normal,
    Ephemeral,
}

pub async fn slash_setup(http: &Http) -> Result<(), Box<dyn Error + Send + Sync>> {
    global_slash("father", "Summon the father", vec![], http).await?;
    global_slash("cock", "cock ?!?!?!?!", vec![], http).await?;
    command_clear_setup(http).await?;
    command_memes_setup(http).await?;
    Ok(())
}

// TODO same as above
// TODO better error handlng
// TODO array not found -> subreddit not found
pub async fn slash_react(http: &Http, command: &ApplicationCommandInteraction) {
    let mut guild_channel = None;

    if let Channel::Guild(channel) = command.channel_id.to_channel(http).await.expect("channel") {
        guild_channel = Some(channel);
    }

    match command.data.name.as_str() {
        "cock" => {
            interact(
                http,
                "I wanna sex so bad <:bite:938339213382942771>",
                command,
                ResponseType::Normal,
            )
            .await
        }
        "father" => {
            interact(
                http,
                "Father <@!710342832145039390> <:bite:938339213382942771>",
                command,
                ResponseType::Normal,
            )
            .await
        }
        "clear" => {
            match command_clear(command, http, guild_channel.expect("empty guild channel")).await {
                Ok(v) | Err(v) => interact(http, v, command, ResponseType::Ephemeral).await,
            }
        }
        // TODO Errors should tell you to report the error the me
        "memes" => match command_meme(command).await {
            Ok(v) => interact(http, v, command, ResponseType::Normal).await,
            Err(err) => interact(http, err.to_string(), command, ResponseType::Ephemeral).await,
        },
        _ => {
            interact(
                http,
                "Imagine demanding for non-existent commands lol",
                command,
                ResponseType::Normal,
            )
            .await
        }
    };
}

pub async fn global_slash(
    name: &str,
    description: &str,
    options: Vec<CreateApplicationCommandOption>,
    http: &Http,
) -> Result<(), Box<dyn Error + Send + Sync>> {
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
    .await?;

    println!("The command `{}` was registered", commands.name);

    Ok(())
}

async fn interact<T: Display>(
    http: &Http,
    content: T,
    interaction: &ApplicationCommandInteraction,
    response_type: ResponseType,
) {
    let content = content.to_string();
    if let Err(why) = {
        let flag = match response_type {
            ResponseType::Normal => InteractionApplicationCommandCallbackDataFlags::empty(),
            ResponseType::Ephemeral => InteractionApplicationCommandCallbackDataFlags::EPHEMERAL,
        };
        interaction
            .create_interaction_response(http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| message.content(content).flags(flag))
            })
            .await
    } {
        println!("Failed slash command interaction. Reason: {}", why);
    }
}
