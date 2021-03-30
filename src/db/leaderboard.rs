use crate::prelude::*;
use serenity::futures::TryStreamExt;
use serenity::model::prelude::*;

#[derive(Debug)]
pub struct LeaderboardEntry {
    pub user_id: String,
    pub channel_id: String,
    pub points: i64,
}

pub async fn get_user_score(guildid: GuildId, channelid: ChannelId, userid: UserId) -> Result<i64> {
    let mut conn = connect().await?;

    let (gid, cid, uid) = (guildid.to_string(), channelid.to_string(), userid.to_string());
    let result = sqlx::query!(
        "SELECT points FROM leaderboard WHERE guild_id == ?1 AND channel_id == ?2 AND user_id == ?3;",
        gid,
        cid,
        uid
    )
    .fetch_optional(&mut conn)
    .await?;

    match result {
        Some(row) => Ok(row.points),
        None => {
            sqlx::query!(
                "INSERT INTO leaderboard (guild_id, channel_id, user_id, points) VALUES (?1, ?2, ?3, ?4);",
                gid,
                cid,
                uid,
                1
            )
            .execute(&mut conn)
            .await?;
            Ok(1)
        }
    }
}

pub async fn add_user_score(guildid: GuildId, channelid: ChannelId, userid: UserId, amount: i64) -> Result<()> {
    let score = get_user_score(guildid, channelid, userid).await?;
    let mut conn = connect().await?;

    let (change, gid, cid, uid) = (
        score + amount,
        guildid.to_string(),
        channelid.to_string(),
        userid.to_string(),
    );
    sqlx::query!(
        "UPDATE leaderboard SET points = ?1 WHERE guild_id == ?2 AND channel_id == ?3 AND user_id == ?4;",
        change,
        gid,
        cid,
        uid
    )
    .execute(&mut conn)
    .await?;
    Ok(())
}

pub async fn inc_user_score(guildid: GuildId, channelid: ChannelId, userid: UserId) -> Result<()> {
    add_user_score(guildid, channelid, userid, 1).await
}

pub async fn get_single_scores(guildid: GuildId, userid: UserId) -> Result<Vec<LeaderboardEntry>> {
    let mut conn = connect().await?;

    let (gid, uid) = (guildid.to_string(), userid.to_string());
    let mut result = sqlx::query!("SELECT user_id, channel_id, points FROM leaderboard WHERE guild_id == ?1 AND user_id == ?2 ORDER BY points DESC;",
        gid, uid).fetch(&mut conn);

    let mut rows = vec![];
    while let Ok(Some(row)) = result.try_next().await {
        rows.push(LeaderboardEntry {
            user_id: row.user_id,
            channel_id: row.channel_id,
            points: row.points,
        })
    }

    Ok(rows)
}

pub async fn get_user_scores(guildid: GuildId) -> Result<Vec<LeaderboardEntry>> {
    let mut conn = connect().await?;

    let gid = guildid.to_string();
    let mut result = sqlx::query!(r#"SELECT user_id, SUM(points) as "points!: i64" FROM leaderboard WHERE guild_id == ?1 GROUP BY user_id ORDER BY "points!: i64" DESC;"#,
        gid).fetch(&mut conn);

    let mut rows = vec![];
    while let Ok(Some(row)) = result.try_next().await {
        rows.push(LeaderboardEntry {
            user_id: row.user_id,
            channel_id: "".into(),
            points: row.points,
        })
    }

    Ok(rows)
}

pub async fn get_channel_scores(guildid: GuildId, channelid: ChannelId) -> Result<Vec<LeaderboardEntry>> {
    let mut conn = connect().await?;

    let (gid, cid) = (guildid.to_string(), channelid.to_string());
    let mut result = sqlx::query!("SELECT user_id, channel_id, points FROM leaderboard WHERE guild_id == ?1 AND channel_id == ?2 ORDER BY points DESC;",
        gid, cid).fetch(&mut conn);

    let mut rows = vec![];
    while let Ok(Some(row)) = result.try_next().await {
        rows.push(LeaderboardEntry {
            user_id: row.user_id,
            channel_id: row.channel_id,
            points: row.points,
        })
    }

    Ok(rows)
}
