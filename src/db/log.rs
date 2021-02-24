use crate::prelude::*;
use serenity::{model::prelude::*, prelude::*};

pub async fn check_log_type(log_type: LogType, guildid: GuildId) -> Result<()> {
    let _ = get_log_channel(guildid).await?;
    let enabled_log_type = get_log_type(guildid).await?;
    let converted_log_type = log_type as i64;

    if enabled_log_type & converted_log_type != converted_log_type {
        return Err(anyhow!("Log type disabled."));
    }

    Ok(())
}

pub async fn log_channel_say(ctx: &Context, guildid: GuildId, message: &str) -> Result<()> {
    let log_channel = get_log_channel(guildid).await?;
    log_channel
        .send_message(ctx, |m| {
            m.content(message);
            m.allowed_mentions(|am| {
                am.empty_parse();
                am
            });
            m
        })
        .await?;
    Ok(())
}

pub async fn get_log_channel(guildid: GuildId) -> Result<ChannelId> {
    let mut conn = connect().await?;
    let gid = guildid.to_string();
    let row = sqlx::query!("SELECT channel_id FROM log WHERE guild_id == ?1;", gid)
        .fetch_one(&mut conn)
        .await?;
    Ok(ChannelId(row.channel_id.parse().unwrap()))
}

pub async fn get_log_type(guildid: GuildId) -> Result<i64> {
    let mut conn = connect().await?;
    let gid = guildid.to_string();
    let row = sqlx::query!("SELECT log_type FROM log WHERE guild_id == ?1;", gid)
        .fetch_one(&mut conn)
        .await?;
    Ok(row.log_type.parse().unwrap())
}
