use crate::prelude::*;
use serenity::model::prelude::*;

pub async fn get_user_channel_score(guildid: GuildId, channelid: ChannelId, userid: UserId) -> Result<i64> {
    let conn = connect()?;
    let mut s = conn.prepare("SELECT points FROM leaderboard WHERE guild_id == ?1 AND channel_id == ?2 AND user_id == ?3;")?;
    let result = s.query_row(&[guildid.to_string(), channelid.to_string(), userid.to_string()], |r| r.get(0));
    if let Err(rusqlite::Error::QueryReturnedNoRows) = result {
        let mut s = conn.prepare("INSERT INTO leaderboard (guild_id, channel_id, user_id, points) VALUES (?1, ?2, ?3, ?4);")?;
        s.execute(&[guildid.to_string(), channelid.to_string(), userid.to_string(), 1.to_string()])?;
        return Ok(1)
    };

    Ok(result?)
}

pub async fn add_user_channel_score(
    guildid: GuildId,
    channelid: ChannelId,
    userid: UserId,
    amount: i64,
) -> Result<i64> {
    let score = get_user_channel_score(guildid, channelid, userid).await?;
    let conn = connect()?;
    let mut s = conn.prepare("UPDATE leaderboard SET points = ?1 WHERE guild_id == ?2 AND channel_id == ?3 AND user_id == ?4;")?;
    let score = s.query_row(&[(score + amount).to_string(), guildid.to_string(), channelid.to_string(), userid.to_string()], |r| r.get(0))?;
    Ok(score)
}

pub async fn inc_user_channel_score(guildid: GuildId, channelid: ChannelId, userid: UserId) -> Result<i64> {
    add_user_channel_score(guildid, channelid, userid, 1).await
}

pub async fn get_user_total_scores(guildid: GuildId) -> Result<Vec<LeaderboardEntry>> {
    let conn = connect()?;
    let mut s = conn.prepare("SELECT user_id, SUM(points) as points FROM leaderboard WHERE guild_id == ?1 GROUP BY user_id ORDER BY points DESC;")?;
    let result = s.query_map(&[guildid.to_string()], |r| {
        Ok(LeaderboardEntry {
            user_id: r.get(0)?,
            points: r.get(1)?,
        })
    })?;

    let res: Result<Vec<LeaderboardEntry>, _> = result.into_iter().collect();
    Ok(res?)
}

pub async fn get_user_channel_scores(
    guildid: GuildId,
    channelid: ChannelId,
) -> Result<Vec<LeaderboardEntry>> {
    let conn = connect()?;
    let mut s = conn.prepare("SELECT user_id, points FROM leaderboard WHERE guild_id == ?1 AND channel_id == ?2 ORDER BY points DESC;")?;
    let result = s.query_map(&[guildid.to_string(), channelid.to_string()], |r| {
        Ok(LeaderboardEntry {
            user_id: r.get(0)?,
            points: r.get(1)?,
        })
    })?;

    let res: Result<Vec<LeaderboardEntry>, _> = result.into_iter().collect();
    Ok(res?)
}
