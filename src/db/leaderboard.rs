use crate::prelude::*;
use serenity::model::prelude::*;

#[derive(Debug)]
pub struct LeaderboardEntry {
    pub user_id: String,
    pub channel_id: String,
    pub points: i64,
}

pub async fn get_user_score(guildid: GuildId, channelid: ChannelId, userid: UserId) -> Result<i64> {
    let conn = connect()?;
    let mut s =
        conn.prepare("SELECT points FROM leaderboard WHERE guild_id == ?1 AND channel_id == ?2 AND user_id == ?3;")?;
    let result = s.query_row(
        params![guildid.to_string(), channelid.to_string(), userid.to_string()],
        |r| r.get(0),
    );
    if let Err(rusqlite::Error::QueryReturnedNoRows) = result {
        let mut s =
            conn.prepare("INSERT INTO leaderboard (guild_id, channel_id, user_id, points) VALUES (?1, ?2, ?3, ?4);")?;
        s.execute(params![
            guildid.to_string(),
            channelid.to_string(),
            userid.to_string(),
            1,
        ])?;
        return Ok(1);
    };

    Ok(result?)
}

pub async fn add_user_score(guildid: GuildId, channelid: ChannelId, userid: UserId, amount: i64) -> Result<i64> {
    let score = get_user_score(guildid, channelid, userid).await?;
    let conn = connect()?;
    let mut s = conn
        .prepare("UPDATE leaderboard SET points = ?1 WHERE guild_id == ?2 AND channel_id == ?3 AND user_id == ?4;")?;
    let score = s.query_row(
        params![
            (score + amount),
            guildid.to_string(),
            channelid.to_string(),
            userid.to_string(),
        ],
        |r| r.get(0),
    )?;
    Ok(score)
}

pub async fn inc_user_score(guildid: GuildId, channelid: ChannelId, userid: UserId) -> Result<i64> {
    add_user_score(guildid, channelid, userid, 1).await
}

pub async fn get_single_scores(guildid: GuildId, userid: UserId) -> Result<Vec<LeaderboardEntry>> {
    let conn = connect()?;
    let mut s = conn.prepare("SELECT user_id, channel_id, points FROM leaderboard WHERE guild_id == ?1 AND user_id == ?2 ORDER BY points DESC;")?;
    let result = s.query_map(params![guildid.to_string(), userid.to_string()], |r| {
        Ok(LeaderboardEntry {
            user_id: r.get(0)?,
            channel_id: r.get(1)?,
            points: r.get(2)?,
        })
    })?;

    let res: Result<Vec<LeaderboardEntry>, _> = result.into_iter().collect();
    Ok(res?)
}

pub async fn get_user_scores(guildid: GuildId) -> Result<Vec<LeaderboardEntry>> {
    let conn = connect()?;
    let mut s = conn.prepare("SELECT user_id, channel_id, SUM(points) as points FROM leaderboard WHERE guild_id == ?1 GROUP BY user_id ORDER BY points DESC;")?;
    let result = s.query_map(params![guildid.to_string()], |r| {
        Ok(LeaderboardEntry {
            user_id: r.get(0)?,
            channel_id: r.get(1)?,
            points: r.get(2)?,
        })
    })?;

    let res: Result<Vec<LeaderboardEntry>, _> = result.into_iter().collect();
    Ok(res?)
}

pub async fn get_channel_scores(guildid: GuildId, channelid: ChannelId) -> Result<Vec<LeaderboardEntry>> {
    let conn = connect()?;
    let mut s = conn.prepare(
        "SELECT user_id, channel_id, points FROM leaderboard WHERE guild_id == ?1 AND channel_id == ?2 ORDER BY points DESC;",
    )?;
    let result = s.query_map(params![guildid.to_string(), channelid.to_string()], |r| {
        Ok(LeaderboardEntry {
            user_id: r.get(0)?,
            channel_id: r.get(1)?,
            points: r.get(2)?,
        })
    })?;

    let res: Result<Vec<LeaderboardEntry>, _> = result.into_iter().collect();
    Ok(res?)
}
