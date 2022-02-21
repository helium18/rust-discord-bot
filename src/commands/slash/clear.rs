use super::global_slash;
use crate::BotError;
use serenity::http::Http;
use serenity::model::channel::GuildChannel;
use serenity::model::interactions::application_command::{
    ApplicationCommandInteraction, ApplicationCommandInteractionDataOptionValue,
};
use serenity::{
    builder::CreateApplicationCommandOption,
    model::interactions::application_command::ApplicationCommandOptionType,
};

// TODO add restrictions (only admin can abuse)
pub async fn command_clear_setup(http: &Http) -> Result<(), BotError> {
    let mut count = CreateApplicationCommandOption::default();

    count
        .name("count")
        .description("The number of messages to clear")
        .kind(ApplicationCommandOptionType::Integer)
        .min_int_value(1)
        .max_int_value(100)
        .required(true);

    let name = "clear";
    let description = "Clear messages";
    let options = vec![count];

    global_slash(name, description, options, http).await?;

    Ok(())
}

pub async fn command_clear(
    interaction: &ApplicationCommandInteraction,
    http: &Http,
    channel: GuildChannel,
) -> Result<String, String> {
    let option = interaction
        .data
        .options
        .get(0)
        .ok_or("Count is required parameter, ni||nja||")?
        .resolved
        .as_ref()
        .ok_or("lmaoded, expected user object but can't find it.")?;

    match option {
        ApplicationCommandInteractionDataOptionValue::Integer(count) => {
            delete_messages(count, http, &channel).await;
            Ok(format!("Deleted {count} messages"))
        }
        _ => Err("lmaoded".into()),
    }
}

async fn delete_messages(count: &i64, http: &Http, channel: &GuildChannel) {
    let messages = channel
        .messages(http, |x| x.limit(*count as u64))
        .await
        .expect("Failed to get messages")
        .into_iter()
        .map(|msg| msg.id);

    channel
        .delete_messages(http, messages)
        .await
        .expect("Failed to delete messages");
}
