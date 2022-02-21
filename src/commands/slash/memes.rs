use crate::commands::setup::global_slash;
use crate::{BotError, NotFound, COLOR, FOOTER_IMAGE};
use rand::Rng;
use serenity::builder::{CreateApplicationCommandOption, CreateInteractionResponseData};
use serenity::http::Http;
use serenity::model::interactions::application_command::{
    ApplicationCommandInteractionDataOptionValue, ApplicationCommandOptionType,
};
use serenity::model::interactions::InteractionApplicationCommandCallbackDataFlags;
use serenity::model::prelude::application_command::ApplicationCommandInteraction;

struct Meme {
    url: Url,
    subreddit: String,
    title: String,
    is_nsfw: bool,
    permalink: String,
}

enum Url {
    Video(String),
    Image(String),
}

struct Recieve {
    pub meme: Meme,
    pub nsfw_allowed: bool,
}

fn meme(json: serde_json::Value) -> Result<Meme, BotError> {
    let arr = &json["data"]["children"];

    let arr_len = arr
        .as_array()
        .ok_or(BotError::NotFound(NotFound::Subreddit))?
        .len();

    let random_int = rand::thread_rng().gen_range(0..arr_len);

    let head = &json["data"]["children"][random_int]["data"];

    let is_nsfw = head["over_18"]
        .to_string()
        .parse::<bool>()
        .map_err(|err| BotError::ParseError(format!("{:#?}", err)))?;

    let is_video = head["is_video"]
        .to_string()
        .parse::<bool>()
        .map_err(|err| BotError::ParseError(format!("{:#?}", err)))?;

    let url = match is_video {
        true => Url::Video(
            head["media"]["reddit_video"]["scrubber_media_url"]
                .to_string()
                .replace("\"", ""),
        ),
        false => Url::Image(head["url"].to_string().replace("\"", "")),
    };

    let subreddit = head["subreddit"].to_string().replace("\"", "");

    let permalink = format!(
        "https://www.reddit.com{}",
        head["permalink"].to_string().replace("\"", "")
    );

    let title = head["title"].to_string().replace("\"", "");

    Ok(Meme {
        url,
        subreddit,
        title,
        is_nsfw,
        permalink,
    })
}

async fn get_json(url: String) -> Result<serde_json::Value, BotError> {
    let response = reqwest::get(url)
        .await
        .map_err(|err| BotError::JsonFetchFail(format!("{:#?}", err)))?;

    let json = response
        .json::<serde_json::Value>()
        .await
        .map_err(|err| BotError::JsonFetchFail(format!("{:#?}", err)))?;

    Ok(json)
}

pub async fn command_memes_setup(http: &Http) -> Result<(), BotError> {
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
        .filter(|opt| opt.is_some())
        .for_each(|opt| match opt.unwrap() {
            ApplicationCommandInteractionDataOptionValue::Boolean(b) => nsfw = b,
            ApplicationCommandInteractionDataOptionValue::String(s) => {
                subreddit = Some("https://www.reddit.com/r/".to_owned() + &s + ".json")
            }
            _ => {}
        });

    if let Some(subreddit) = subreddit {
        let json = get_json(subreddit).await?;

        let meme = meme(json)?;

        let recieved = Recieve {
            meme,
            nsfw_allowed: nsfw,
        };

        Ok(recieved)
    } else {
        Err(BotError::NotFound(NotFound::Subreddit))
    }
}

pub async fn command_meme(
    interaction: &ApplicationCommandInteraction,
) -> Result<CreateInteractionResponseData, BotError> {
    let mut recieved = get_memes(interaction).await?;
    let mut content = CreateInteractionResponseData::default();

    if !recieved.nsfw_allowed && recieved.meme.is_nsfw {
        recieved.meme.title = "Nsfw detected \\😡".into();
        recieved.meme.url = Url::Image("https://i.imgur.com/4R0eAtX.gif".into());
    }

    match recieved.meme.url {
        Url::Image(img) => {
            content.create_embed(|embed| {
                embed
                    .url(recieved.meme.permalink)
                    .title(format!("**{}**", recieved.meme.title))
                    .image(img)
                    .color(COLOR);

                embed.footer(|footer| {
                    footer.icon_url(FOOTER_IMAGE).text(format!(
                        "Click the title for source.\nSubreddit: r/{}",
                        recieved.meme.subreddit
                    ))
                })
            });
        }
        Url::Video(_) => {
            content.content(recieved.meme.permalink);
        }
    }

    content.flags(InteractionApplicationCommandCallbackDataFlags::empty());

    Ok(content)
}
