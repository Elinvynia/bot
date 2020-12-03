use crate::prelude::*;
use serenity::model::prelude::*;
use sqlx::prelude::*;

pub async fn get_user_money(guildid: GuildId, userid: UserId) -> Result<Money, BotError> {
    let mut conn = connect().await?;
    let gid: i64 = guildid.into();
    let uid: i64 = userid.into();

    match sqlx::query("SELECT money FROM money WHERE guild_id == ?1 AND user_id == ?2;")
        .bind(gid)
        .bind(uid)
        .fetch_one(&mut conn)
        .await
    {
        Ok(row) => {
            let amount: i64 = row.try_get(0)?;
            Ok(Money(amount as u64))
        },
        Err(sqlx::Error::RowNotFound) => {
            sqlx::query("INSERT INTO money (guild_id, user_id) values (?1, ?2);")
                .bind(gid)
                .bind(uid)
                .execute(&mut conn)
                .await?;
            Ok(Money(0))
        },
        Err(e) => {
            Err(e.into())
        }

    }
}

pub async fn set_user_money(guildid: GuildId, userid: UserId, amount: Money) -> Result<(), BotError> {
    let mut conn = connect().await?;
    let gid: i64 = guildid.into();
    let uid: i64 = userid.into();

    sqlx::query("INSERT OR REPLACE INTO money (guild_id, user_id, money) values (?1, ?2, ?3);")
        .bind(gid)
        .bind(uid)
        .bind(amount.0 as i64)
        .execute(&mut conn)
        .await?;

    Ok(())
}

pub async fn add_user_money(guildid: GuildId, userid: UserId, amount: Money) -> Result<(), BotError> {
    let money = get_user_money(guildid, userid).await?;

    let mut conn = connect().await?;
    let gid: i64 = guildid.into();
    let uid: i64 = userid.into();

    sqlx::query("INSERT OR REPLACE INTO money (guild_id, user_id, money) values (?1, ?2, ?3);")
        .bind(gid)
        .bind(uid)
        .bind((money.0 as i64) + (amount.0 as i64))
        .execute(&mut conn)
        .await?;

    Ok(())
}

pub async fn remove_user_money(guildid: GuildId, userid: UserId, amount: Money) -> Result<(), BotError> {
    let money = get_user_money(guildid, userid).await?;

    if amount > money {
        return Err(BotError::NegativeMoney);
    };

    let mut conn = connect().await?;
    let gid: i64 = guildid.into();
    let uid: i64 = userid.into();

    sqlx::query("INSERT OR REPLACE INTO money (guild_id, user_id, money) values (?1, ?2, ?3);")
        .bind(gid)
        .bind(uid)
        .bind((money.0 as i64) - (amount.0 as i64))
        .execute(&mut conn)
        .await?;

    Ok(())
}

pub async fn inc_user_money(guildid: GuildId, userid: UserId) -> Result<(), BotError> {
    add_user_money(guildid, userid, Money(1)).await
}
