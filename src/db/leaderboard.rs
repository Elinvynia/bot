use super::connect;
use crate::data::{db::LeaderboardEntry, error::BotError};
use serenity::model::prelude::*;
use sqlx::prelude::SqliteQueryAs;
use sqlx::prelude::{Cursor, Row};

pub async fn get_user_channel_score(guildid: GuildId, channelid: ChannelId, userid: UserId) -> Result<i64, BotError> {
    let mut conn = connect().await?;
    let result =
        sqlx::query("SELECT points FROM leaderboard WHERE guild_id == ?1 AND channel_id == ?2 AND user_id == ?3;")
            .bind(&guildid.to_string())
            .bind(&channelid.to_string())
            .bind(&userid.to_string())
            .fetch(&mut conn)
            .next()
            .await?
            .ok_or_else(|| BotError::NoRecordYet)?
            .try_get(0)?;

    Ok(result)
}

pub async fn get_user_total_scores(guildid: GuildId) -> Result<Vec<LeaderboardEntry>, BotError> {
    let mut conn = connect().await?;
    let result = sqlx::query_as("SELECT user_id, SUM(points) as points FROM leaderboard WHERE guild_id == ?1 GROUP BY user_id ORDER BY points DESC LIMIT 10;")
        .bind(&guildid.to_string())
        .fetch_all(&mut conn)
        .await?;

    Ok(result)
}

pub async fn get_user_channel_scores(
    guildid: GuildId,
    channelid: ChannelId,
) -> Result<Vec<LeaderboardEntry>, BotError> {
    let mut conn = connect().await?;
    let result = sqlx::query_as("SELECT user_id, points FROM leaderboard WHERE guild_id == ?1 AND channel_id == ?2 ORDER BY points DESC LIMIT 10;")
        .bind(&guildid.to_string())
        .bind(&channelid.to_string())
        .fetch_all(&mut conn)
        .await?;

    Ok(result)
}
