use super::connect;
use crate::data::{db::LogType, error::BotError};
use serenity::{model::prelude::*, prelude::*};
use sqlx::prelude::*;

pub async fn check_log_type(log_type: LogType, guildid: GuildId) -> Result<(), BotError> {
    let _ = get_log_channel(guildid).await?;
    let enabled_log_type = get_log_type(guildid).await?;
    let converted_log_type = log_type as i64;

    if enabled_log_type & converted_log_type != converted_log_type {
        return Err(BotError::LogTypeDisabled);
    }

    Ok(())
}

pub async fn log_channel_say(ctx: &Context, guildid: GuildId, message: &str) -> Result<(), BotError> {
    let log_channel = get_log_channel(guildid).await?;
    log_channel.say(ctx, message).await?;
    Ok(())
}

pub async fn get_log_channel(guildid: GuildId) -> Result<ChannelId, BotError> {
    let mut conn = connect().await?;
    let cid: i64 = sqlx::query("SELECT channel_id FROM log WHERE guild_id == ?1;")
        .bind(&guildid.to_string())
        .fetch(&mut conn)
        .next()
        .await?
        .ok_or_else(|| "Guild not found channel".to_string())?
        .try_get(0)?;
    Ok(ChannelId(cid as u64))
}

pub async fn get_log_type(guildid: GuildId) -> Result<i64, BotError> {
    let mut conn = connect().await?;
    let log_type = sqlx::query("SELECT log_type FROM log WHERE guild_id == ?1;")
        .bind(&guildid.to_string())
        .fetch(&mut conn)
        .next()
        .await?
        .ok_or_else(|| "Guild not found type.".to_string())?
        .try_get(0)?;
    Ok(log_type)
}
