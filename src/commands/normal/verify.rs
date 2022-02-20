use std::error::Error;

use serenity::prelude::Context;

use crate::BotError;

pub struct Details<'a> {
    pub message_id: u64,
    pub channel_id: u64,
    pub role_name: &'a str,
}

pub async fn verify(ctx: &Context, conf: Details<'_>) -> Result<(), Box<dyn Error>> {
    let http = &ctx.http;
    let message = http.get_message(conf.channel_id, conf.message_id).await?;

    // Listen for reaction
    if let Some(reaction) = message.await_reaction(&ctx).await {
        let reaction = reaction.as_inner_ref();
        let emoji = &reaction.emoji;

        // Check for the correct emoji
        if emoji.as_data().as_str() == "✅" {
            let user = reaction.user(&http).await?;

            let guild = message
                .channel_id
                .to_channel(&http)
                .await?
                .guild()
                .ok_or_else(|| BotError::NotFound("Guild not found!".into()))?
                .guild_id;

            let cached_guild = guild
                .to_guild_cached(&ctx)
                .await
                .ok_or_else(|| BotError::NotFound("Cached Guild not found!".into()))?;

            let role = cached_guild
                .role_by_name(conf.role_name)
                .ok_or_else(|| BotError::NotFound("Role not found".into()))?;

            match user.has_role(&http, guild, role).await? {
                true => println!("Verfied user `{}` tried to verify.", user.name),
                false => {
                    http.add_member_role(guild.into(), user.id.into(), role.id.into())
                        .await?;
                    println!("User `{}` has been verified.", user.name);
                }
            }
        }
    }

    Ok(())
}
