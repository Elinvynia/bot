use serenity::{model::prelude::*};
use crate::data::{db::LeaderboardEntry, error::BotError};
use super::get_db;

pub fn get_user_channel_score(
    guildid: &GuildId,
    channelid: &ChannelId,
    userid: &UserId,
) -> Result<i64, BotError> {
    let conn = get_db()?;
    let mut statement = conn.prepare(
        "SELECT points FROM leaderboard WHERE guild_id == ?1 AND channel_id == ?2 AND user_id == ?3;",
    )?;
    let mut rows = statement.query(&[
        &guildid.as_u64().to_string(),
        &channelid.as_u64().to_string(),
        &userid.as_u64().to_string(),
    ])?;
    Ok(rows.next()?.ok_or("No record yet.".to_string())?.get(0)?)
}

pub fn get_user_total_scores(guildid: &GuildId) -> Result<Vec<LeaderboardEntry>, BotError> {
    let guild_id = guildid.as_u64().to_string();
    let conn = get_db()?;
    let mut statement =
        conn.prepare("SELECT user_id, SUM(points) as points FROM leaderboard WHERE guild_id == ?1 GROUP BY user_id ORDER BY points DESC LIMIT 10;")?;
    let result_iter = statement.query_map(&[&guild_id], |row| {
        Ok(LeaderboardEntry {
            user_id: row.get(0)?,
            points: row.get(1)?,
        })
    })?;

    let mut result = Vec::new();
    for x in result_iter {
        result.push(x?);
    }

    Ok(result)
}

pub fn get_user_channel_scores(
    guildid: &GuildId,
    channelid: &ChannelId,
) -> Result<Vec<LeaderboardEntry>, BotError> {
    let guild_id = guildid.as_u64().to_string();
    let channel_id = channelid.as_u64().to_string();
    let conn = get_db()?;
    let mut statement =
        conn.prepare("SELECT user_id, points FROM leaderboard WHERE guild_id == ?1 AND channel_id == ?2 ORDER BY points DESC LIMIT 10;")?;
    let result_iter = statement.query_map(&[&guild_id, &channel_id], |row| {
        Ok(LeaderboardEntry {
            user_id: row.get(0)?,
            points: row.get(1)?,
        })
    })?;

    let mut result = Vec::new();
    for x in result_iter {
        result.push(x?);
    }

    Ok(result)
}
