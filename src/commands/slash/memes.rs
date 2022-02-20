use crate::commands::setup::global_slash;
use crate::BotError;
use rand::Rng;
use serenity::builder::CreateApplicationCommandOption;
use serenity::http::Http;
use serenity::model::interactions::application_command::{
    ApplicationCommandInteractionDataOptionValue, ApplicationCommandOptionType,
};
use serenity::model::prelude::application_command::ApplicationCommandInteraction;
use std::error::Error;

struct Meme {
    url: Url,
    subreddit: String,
    title: String,
    is_nsfw: bool,
}

enum Url {
    Video(String),
    Image(String),
}

struct Recieve {
    pub meme: Meme,
    pub nsfw_allowed: bool,
}

fn meme(json: serde_json::Value) -> Result<Meme, Box<dyn Error + Send + Sync>> {
    let arr = &json["data"]["children"];
    let arr_len = arr
        .as_array()
        .ok_or_else(|| BotError::NotFound("Array was not found".to_string()))?
        .len();
    let random_int = rand::thread_rng().gen_range(0..arr_len);
    let head = &json["data"]["children"][random_int]["data"];

    let is_nsfw = head["over_18"].to_string().parse::<bool>()?;

    let is_video = head["is_video"].to_string().parse::<bool>()?;

    let url = match is_video {
        true => Url::Video(
            head["media"]["reddit_video"]["scrubber_media_url"]
                .to_string()
                .replace("\"", ""),
        ),
        false => Url::Image(head["url"].to_string().replace("\"", "")),
    };

    let subreddit = head["subreddit"].to_string().replace("\"", "");

    let title = head["title"].to_string().replace("\"", "");

    Ok(Meme {
        url,
        subreddit,
        title,
        is_nsfw,
    })
}

async fn get_json(url: String) -> Result<serde_json::Value, Box<dyn Error + Send + Sync>> {
    let response = reqwest::get(url).await?;
    let json = response.json::<serde_json::Value>().await?;
    Ok(json)
}

pub async fn command_memes_setup(http: &Http) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut subreddit_opt = CreateApplicationCommandOption::default();

    subreddit_opt
        .name("subreddit")
        .description("The subreddit to fetch memes from.")
        .kind(ApplicationCommandOptionType::String)
        .required(true);

    let mut nsfw_opt = CreateApplicationCommandOption::default();

    nsfw_opt
        .name("nsfw")
        .description("Allow haram")
        .kind(ApplicationCommandOptionType::Boolean)
        .required(false);

    let name = "memes";
    let description = "Get memes!";
    let options = vec![subreddit_opt, nsfw_opt];

    global_slash(name, description, options, http).await?;

    Ok(())
}

async fn get_memes(interaction: &ApplicationCommandInteraction) -> Result<Recieve, BotError> {
    // defaulting nsfw to false
    let mut nsfw = false;
    let mut subreddit = None;

    interaction
        .data
        .options
        .clone()
        .into_iter()
        .map(|opt| opt.resolved)
        // .filter(|opt| opt.is_some())
        .for_each(|opt| match opt.unwrap() {
            ApplicationCommandInteractionDataOptionValue::Boolean(b) => nsfw = b,
            ApplicationCommandInteractionDataOptionValue::String(s) => {
                subreddit = Some("https://www.reddit.com/r/".to_owned() + &s + ".json")
            }
            _ => {}
        });

    if let Some(subreddit) = subreddit {
        let json = get_json(subreddit)
            .await
            .map_err(|err| BotError::JsonFetchFail(err))?;

        let meme = meme(json).map_err(|err| BotError::MemeFetchFail(err))?;

        let recieved = Recieve {
            meme,
            nsfw_allowed: nsfw,
        };

        Ok(recieved)
    } else {
        Err(BotError::NotFound(
            "Failed to get details about subreddit and nsfw status".into(),
        ))
    }
}

// TODO Return embed instead
// TODO Make program look better (url especially)
pub async fn command_meme(interaction: &ApplicationCommandInteraction) -> Result<String, BotError> {
    let recieved = get_memes(interaction).await?;

    let url = match recieved.meme.url {
        Url::Image(url) | Url::Video(url) => url,
    };

    if !recieved.nsfw_allowed && recieved.meme.is_nsfw {
        // TODO add subreddit info
        Ok("https://tenor.com/view/haram-heisenberg-gif-20680378".into())
    } else {
        Ok(format!(
            "Title: {}\nUrl: {}\nSubreddit: {}",
            recieved.meme.title, url, recieved.meme.subreddit
        ))
    }
}
