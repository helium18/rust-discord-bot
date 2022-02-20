use serenity::prelude::Context;

use crate::{BotError, NotFound};

pub struct Details<'a> {
    pub message_id: u64,
    pub channel_id: u64,
    pub role_name: &'a str,
}

pub async fn verify(ctx: &Context, conf: Details<'_>) -> Result<(), BotError> {
    let http = &ctx.http;
    let message = http
        .get_message(conf.channel_id, conf.message_id)
        .await
        .map_err(|err| BotError::MemeFetchFail(format!("{:#?}", err)))?;

    // Listen for reaction
    if let Some(reaction) = message.await_reaction(&ctx).await {
        let reaction = reaction.as_inner_ref();
        let emoji = &reaction.emoji;

        // Check for the correct emoji
        if emoji.as_data().as_str() == "✅" {
            let user = reaction
                .user(&http)
                .await
                .map_err(|err| BotError::UserFetchFail(format!("{:#?}", err)))?;

            let guild = message
                .channel_id
                .to_channel(&http)
                .await
                .map_err(|err| BotError::ChannelIdToChannelConvertFail(format!("{:#?}", err)))?
                .guild()
                .ok_or_else(|| BotError::NotFound(NotFound::GuildChannel))?
                .guild_id;

            let cached_guild = guild
                .to_guild_cached(&ctx)
                .await
                .ok_or_else(|| BotError::NotFound(NotFound::CachedGuild))?;

            let role = cached_guild
                .role_by_name(conf.role_name)
                .ok_or_else(|| BotError::NotFound(NotFound::Role(conf.role_name.into())))?;

            match user
                .has_role(&http, guild, role)
                .await
                .map_err(|err| BotError::UserRoleCheckFail(format!("{:#?}", err)))?
            {
                true => {
                    println!("Verfied user `{}` tried to verify.", user.name);
                    Ok(())
                }
                false => {
                    match http
                        .add_member_role(guild.into(), user.id.into(), role.id.into())
                        .await
                    {
                        Ok(_) => {
                            println!("User `{}` has been verified.", user.name);
                            Ok(())
                        }
                        Err(err) => Err(BotError::RoleAssignmentFailure(format!("{:#?}", err))),
                    }
                }
            }
        } else {
            Ok(())
        }
    } else {
        Ok(())
    }
}
