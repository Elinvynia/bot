use crate::prelude::*;
use serenity::model::prelude::*;
use sqlx::prelude::{Cursor, Row, SqliteQueryAs};

pub async fn get_user_channel_score(guildid: GuildId, channelid: ChannelId, userid: UserId) -> Result<i64, BotError> {
    let mut conn = connect().await?;
    let result = match sqlx::query(
        "SELECT points FROM leaderboard WHERE guild_id == ?1 AND channel_id == ?2 AND user_id == ?3;",
    )
    .bind(&guildid.to_string())
    .bind(&channelid.to_string())
    .bind(&userid.to_string())
    .fetch(&mut conn)
    .next()
    .await?
    {
        Some(row) => row.try_get(0)?,
        None => {
            sqlx::query("INSERT INTO leaderboard (guild_id, channel_id, user_id, points) VALUES (?1, ?2, ?3, ?4);")
                .bind(&guildid.to_string())
                .bind(&channelid.to_string())
                .bind(&userid.to_string())
                .bind(1)
                .execute(&mut conn)
                .await?;
            1
        }
    };

    Ok(result)
}

pub async fn add_user_channel_score(
    guildid: GuildId,
    channelid: ChannelId,
    userid: UserId,
    amount: i64,
) -> Result<i64, BotError> {
    let score = get_user_channel_score(guildid, channelid, userid).await?;
    let mut conn = connect().await?;
    let result =
        sqlx::query("UPDATE leaderboard SET points = ?1 WHERE guild_id == ?2 AND channel_id == ?3 AND user_id == ?4;")
            .bind(score + amount)
            .bind(&guildid.to_string())
            .bind(&channelid.to_string())
            .bind(&userid.to_string())
            .fetch(&mut conn)
            .next()
            .await?
            .ok_or(BotError::NoRecordYet)?
            .try_get(0)?;

    Ok(result)
}

pub async fn inc_user_channel_score(guildid: GuildId, channelid: ChannelId, userid: UserId) -> Result<i64, BotError> {
    add_user_channel_score(guildid, channelid, userid, 1).await
}

pub async fn get_user_total_scores(guildid: GuildId) -> Result<Vec<LeaderboardEntry>, BotError> {
    let mut conn = connect().await?;
    let result = sqlx::query_as("SELECT user_id, SUM(points) as points FROM leaderboard WHERE guild_id == ?1 GROUP BY user_id ORDER BY points DESC;")
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
    let result = sqlx::query_as(
        "SELECT user_id, points FROM leaderboard WHERE guild_id == ?1 AND channel_id == ?2 ORDER BY points DESC;",
    )
    .bind(&guildid.to_string())
    .bind(&channelid.to_string())
    .fetch_all(&mut conn)
    .await?;

    Ok(result)
}
