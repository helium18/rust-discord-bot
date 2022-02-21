use crate::commands::setup::global_slash;
use crate::{BotError, COLOR, FOOTER_IMAGE};
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
// TODO fix memes setup `options` shouldn't be a vec, instead a mutable opt
pub async fn command_embed_setup(http: &Http) -> Result<(), BotError> {
    let mut embed_opt = CreateApplicationCommandOption::default();

    let title_opt = embed_opt
        .name("title")
        .description("Title for the embed")
        .kind(ApplicationCommandOptionType::String)
        .required(false)
        .to_owned();

    let description_opt = embed_opt
        .name("description")
        .description("Description for the embed. Use `$n` for a line-break")
        .kind(ApplicationCommandOptionType::String)
        .required(true)
        .to_owned();

    let footer_opt = embed_opt
        .name("footer")
        .description("Footer for the embed")
        .kind(ApplicationCommandOptionType::String)
        .required(false)
        .to_owned();

    let link_opt = embed_opt
        .name("link")
        .description("Link for the title")
        .kind(ApplicationCommandOptionType::String)
        .required(false)
        .to_owned();

    let image_opt = embed_opt
        .name("image")
        .description("Image to include. Should be an `https` link")
        .kind(ApplicationCommandOptionType::String)
        .required(false)
        .to_owned();

    let name = "embed";
    let description = "Create an embed";
    let options = vec![description_opt, footer_opt, link_opt, image_opt, title_opt];

    global_slash(name, description, options, http).await?;

    Ok(())
}

pub async fn command_embed(
    interaction: &ApplicationCommandInteraction,
    http: &Http,
    channel: GuildChannel,
) -> Result<(), BotError> {
    channel
        .send_message(http, |msg| {
            msg.add_embed(|embed| {
                interaction
                    .data
                    .options
                    .clone()
                    .into_iter()
                    .map(|opt| (opt.name, opt.resolved))
                    .filter(|(_, opt_val)| opt_val.is_some())
                    .for_each(|(opt_name, opt_val)| {
                        if let ApplicationCommandInteractionDataOptionValue::String(opt_val) =
                            opt_val.unwrap()
                        {
                            match opt_name.as_str() {
                                "title" => embed.title(opt_val),
                                "description" => {
                                    let description = opt_val.replace("$n", "\n");
                                    embed.description(description)
                                }
                                "footer" => {
                                    embed.footer(|ftr| ftr.text(opt_val).icon_url(FOOTER_IMAGE))
                                }
                                "link" => embed.url(opt_val),
                                "image" => embed.image(opt_val),
                                _ => embed,
                            };
                        }
                    });
                embed.color(COLOR)
            });
            msg
        })
        .await
        .map_err(|err| BotError::MessageSendFail(err.to_string()))?;
    Ok(())
}
