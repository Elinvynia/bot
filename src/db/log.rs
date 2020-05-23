use super::connect;
use crate::data::error::BotError;
use serenity::{model::prelude::*, prelude::*};
use sqlx::prelude::*;

pub async fn get_log_channel(_ctxx: &Context, guildid: GuildId) -> Result<ChannelId, BotError> {
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

pub async fn get_log_type(_ctxx: &Context, guildid: GuildId) -> Result<i64, BotError> {
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
