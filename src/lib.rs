use std::{
    error::Error,
    fmt::{Debug, Display},
    ops::Deref,
};

use serenity::utils::Color;

#[path = "./commands/commands.rs"]
pub mod commands;

#[path = "./utils/utils.rs"]
pub mod utils;

pub const FOOTER_IMAGE: &str = "https://i.imgur.com/TRbBsKu.jpeg";
pub const COLOR: Color = Color::from_rgb(88, 101, 242);

trait Logging: Debug + Send + Sync {}
trait BotErrorLogging: Error + Send + Send + Debug {}

// TODO
#[derive(Debug)]
pub enum BotError {
    NotFound(NotFound),
    JsonFetchFail(String),
    MemeFetchFail(String),
    ParseError(String),
    MessageFetchFail(String),
    ChannelIdToChannelConvertFail(String),
    RoleAssignmentFailure(String),
    UserFetchFail(String),
    UserRoleCheckFail(String),
    GlobalSlashCommandsCreationFail(String),
    SlashCommandInteractionFail(String),
    DeleteMessageFail(String),
}

#[derive(Debug)]
pub enum NotFound {
    MessageComponentInteraction,
    GuildID,
    Subreddit,
    GuildChannel,
    Guild,
    ReactionAction,
    CachedGuild,
    Role(String),
}

// TODO add backtrace for error messages or prolly run all programs with rust backtrace = 1
impl Display for BotError {
    fn fmt(
        self: &BotError,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        let message = match self {
            BotError::NotFound(err) => format!("The `{}` was not found", err.deref()),
            BotError::JsonFetchFail(err) => format!("Failed to fetch JSON:\n```{}```", err),
            BotError::MemeFetchFail(err) => format!("Failed to fetch Meme:\n```{}```", err),
            BotError::ParseError(err) => format!("Failed to parse `{}`", err.deref()),
            BotError::MessageFetchFail(err) => format!("Failed to fetch message: ```{}```", err),
            BotError::ChannelIdToChannelConvertFail(err) => {
                format!(
                    "Failed to convert `Channel ID` to `Channel`:\n```{}```",
                    err
                )
            }
            BotError::RoleAssignmentFailure(err) => {
                format!("Failed to assign `Role` to `User`:\n```{}```", err)
            }
            BotError::UserFetchFail(err) => format!("Failed to fetch the `User`:\n```{}```", err),
            BotError::UserRoleCheckFail(err) => {
                format!("Failed to check `User`'s role:\n```{}```", err)
            }
            BotError::GlobalSlashCommandsCreationFail(err) => {
                format!(
                    "Failed to create the `Global Slash Command`:\n```{}```",
                    err
                )
            }
            BotError::SlashCommandInteractionFail(err) => {
                format!("Failed Slash Command Interaction:\n```{}```", err)
            }
            BotError::DeleteMessageFail(err) => {
                format!("Failed to delete the `Message`:\n```{}```", err)
            }
        };

        write!(f, "{message}")
    }
}

impl Error for BotError {}

impl Display for NotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let content = match self {
            NotFound::MessageComponentInteraction => {
                String::from("`Message Component Interaction`")
            }
            NotFound::GuildID => String::from("`Guild ID`"),
            NotFound::Subreddit => String::from("`Subreddit`"),
            NotFound::GuildChannel => String::from("`Guild Channel`"),
            NotFound::Guild => String::from("`Guild`"),
            NotFound::ReactionAction => String::from("`Reaction's Action`"),
            NotFound::CachedGuild => String::from("`Cached Guild`"),
            NotFound::Role(which) => format!("`{which}` role"),
        };

        write!(f, "{content}")
    }
}
