use super::get_db;
use crate::data::{db::LeaderboardEntry, error::BotError};
use serenity::{model::prelude::*, prelude::*};
use sqlx::prelude::{Cursor, Row};
use sqlx::prelude::SqliteQueryAs;

pub async fn get_user_channel_score(
    ctx: &Context,
    guildid: GuildId,
    channelid: ChannelId,
    userid: UserId,
) -> Result<i64, BotError> {
    let mut conn = get_db(ctx).await?;
    let result = sqlx::query("SELECT points FROM leaderboard WHERE guild_id == ?1 AND channel_id == ?2 AND user_id == ?3;")
        .bind(&guildid.to_string())
        .bind(&channelid.to_string())
        .bind(&userid.to_string())
        .fetch(&mut conn)
        .next()
        .await?
        .ok_or_else(|| "No record yet.".to_string())?
        .try_get(0)?;

    Ok(result)
}

pub async fn get_user_total_scores(
    ctx: &Context,
    guildid: GuildId,
) -> Result<Vec<LeaderboardEntry>, BotError> {
    let mut conn = get_db(ctx).await?;
    let result = sqlx::query_as("SELECT user_id, SUM(points) as points FROM leaderboard WHERE guild_id == ?1 GROUP BY user_id ORDER BY points DESC LIMIT 10;")
        .bind(&guildid.to_string())
        .fetch_all(&mut conn)
        .await?;

    Ok(result)
}

pub async fn get_user_channel_scores(
    ctx: &Context,
    guildid: GuildId,
    channelid: ChannelId,
) -> Result<Vec<LeaderboardEntry>, BotError> {
    let mut conn = get_db(ctx).await?;
    let result = sqlx::query_as("SELECT user_id, points FROM leaderboard WHERE guild_id == ?1 AND channel_id == ?2 ORDER BY points DESC LIMIT 10;")
        .bind(&guildid.to_string())
        .bind(&channelid.to_string())
        .fetch_all(&mut conn)
        .await?;

    Ok(result)
}
