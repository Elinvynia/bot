use super::get_db;
use crate::data::error::BotError;
use serenity::{model::prelude::*, prelude::*};
use sqlx::prelude::*;

pub async fn get_log_channel(ctx: &Context, guildid: GuildId) -> Result<ChannelId, BotError> {
    let mut conn = get_db(ctx).await?;
    let cid: i64 = sqlx::query("SELECT channel_id FROM log WHERE guild_id == ?1;")
        .bind(&guildid.to_string())
        .fetch(&mut conn)
        .next()
        .await?
        .ok_or_else(|| "Guild not found.".to_string())?
        .try_get(0)?;
    Ok(ChannelId(cid as u64))
}

pub async fn get_log_type(ctx: &Context, guildid: GuildId) -> Result<i64, BotError> {
    let mut conn = get_db(ctx).await?;
    let log_type = sqlx::query("SELECT log_type FROM log WHERE guild_id == ?1;")
        .bind(&guildid.to_string())
        .fetch(&mut conn)
        .next()
        .await?
        .ok_or_else(|| "Guild not found.".to_string())?
        .try_get(0)?;
    Ok(log_type)
}
