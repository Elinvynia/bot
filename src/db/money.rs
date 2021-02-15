use crate::prelude::*;
use serenity::model::prelude::*;

pub async fn get_user_money(guildid: GuildId, userid: UserId) -> Result<i64> {
    let conn = connect()?;
    let gid: i64 = guildid.into();
    let uid: i64 = userid.into();

    let mut s = conn.prepare("SELECT money FROM money WHERE guild_id == ?1 AND user_id == ?2;")?;
    let r = s.query_row(params![gid, uid], |r| r.get(0));

    if let Err(rusqlite::Error::QueryReturnedNoRows) = r {
        let mut s = conn.prepare("INSERT INTO money (guild_id, user_id) values (?1, ?2);")?;
        s.execute(params![gid, uid])?;
        return Ok(0)
    };

    Ok(r?)
}

pub async fn set_user_money(guildid: GuildId, userid: UserId, amount: i64) -> Result<()> {
    let conn = connect()?;
    let gid: i64 = guildid.into();
    let uid: i64 = userid.into();

    let mut s = conn.prepare("INSERT OR REPLACE INTO money (guild_id, user_id, money) values (?1, ?2, ?3);")?;
    s.execute(params![gid, uid, amount])?;

    Ok(())
}

pub async fn add_user_money(guildid: GuildId, userid: UserId, amount: i64) -> Result<()> {
    let money = get_user_money(guildid, userid).await?;

    let conn = connect()?;
    let gid: i64 = guildid.into();
    let uid: i64 = userid.into();

    let mut s = conn.prepare("INSERT OR REPLACE INTO money (guild_id, user_id, money) values (?1, ?2, ?3);")?;
    s.execute(params![gid, uid, money + amount])?;

    Ok(())
}

pub async fn remove_user_money(guildid: GuildId, userid: UserId, amount: i64) -> Result<()> {
    let money = get_user_money(guildid, userid).await?;

    if amount > money {
        return Err(anyhow!("Negative money"));
    };

    let conn = connect()?;
    let gid: i64 = guildid.into();
    let uid: i64 = userid.into();

    let mut s = conn.prepare("INSERT OR REPLACE INTO money (guild_id, user_id, money) values (?1, ?2, ?3);")?;
    s.execute(params![gid, uid, money - amount])?;

    Ok(())
}

pub async fn inc_user_money(guildid: GuildId, userid: UserId) -> Result<()> {
    add_user_money(guildid, userid, 1).await
}
