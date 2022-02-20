use std::{error::Error, fmt::Display};

#[path = "./commands/commands.rs"]
pub mod commands;

#[derive(Debug)]
pub enum BotError {
    NotFound(String),
    JsonFetchFail(Box<dyn Error + Send + Sync>),
    MemeFetchFail(Box<dyn Error + Send + Sync>),
}

impl Display for BotError {
    fn fmt(
        self: &BotError,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        let message = match self {
            BotError::NotFound(err) => err.clone(),
            BotError::JsonFetchFail(err) => {
                "Failed to get JSON\nReason:".to_owned() + &err.to_string()
            }
            BotError::MemeFetchFail(err) => {
                "Failed to fetch Meme\nReason:".to_owned() + &err.to_string()
            }
        };

        write!(f, "{message}")
    }
}

impl Error for BotError {}
