use crate::prelude::*;
use serenity::model::prelude::*;

pub async fn get_user_money(guildid: GuildId, userid: UserId) -> Result<i64> {
    let mut conn = connect().await?;
    let gid: i64 = guildid.into();
    let uid: i64 = userid.into();

    let result = sqlx::query!(
        "SELECT money FROM money WHERE guild_id == ?1 AND user_id == ?2;",
        gid,
        uid
    )
    .fetch_optional(&mut conn)
    .await?;

    match result {
        Some(row) => Ok(row.money),
        None => {
            sqlx::query!("INSERT INTO money (guild_id, user_id) values (?1, ?2);", gid, uid)
                .execute(&mut conn)
                .await?;
            Ok(0)
        }
    }
}

pub async fn set_user_money(guildid: GuildId, userid: UserId, amount: i64) -> Result<()> {
    let mut conn = connect().await?;
    let gid: i64 = guildid.into();
    let uid: i64 = userid.into();

    sqlx::query!(
        "INSERT OR REPLACE INTO money (guild_id, user_id, money) values (?1, ?2, ?3);",
        gid,
        uid,
        amount
    )
    .execute(&mut conn)
    .await?;

    Ok(())
}

pub async fn add_user_money(guildid: GuildId, userid: UserId, amount: i64) -> Result<()> {
    let money = get_user_money(guildid, userid).await?;

    let mut conn = connect().await?;
    let gid: i64 = guildid.into();
    let uid: i64 = userid.into();

    let change = money + amount;
    sqlx::query!(
        "INSERT OR REPLACE INTO money (guild_id, user_id, money) values (?1, ?2, ?3);",
        gid,
        uid,
        change
    )
    .execute(&mut conn)
    .await?;

    Ok(())
}

pub async fn remove_user_money(guildid: GuildId, userid: UserId, amount: i64) -> Result<()> {
    let money = get_user_money(guildid, userid).await?;

    if amount > money {
        return Err(anyhow!("Negative money"));
    };

    let mut conn = connect().await?;
    let gid: i64 = guildid.into();
    let uid: i64 = userid.into();

    let change = money - amount;
    sqlx::query!(
        "INSERT OR REPLACE INTO money (guild_id, user_id, money) values (?1, ?2, ?3);",
        gid,
        uid,
        change
    )
    .execute(&mut conn)
    .await?;

    Ok(())
}

pub async fn inc_user_money(guildid: GuildId, userid: UserId) -> Result<()> {
    add_user_money(guildid, userid, 1).await
}
