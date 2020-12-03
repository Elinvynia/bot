use crate::prelude::*;
use serenity::model::prelude::*;
use sqlx::prelude::*;

pub async fn get_prefix(guildid: GuildId) -> Result<String, BotError> {
    let mut conn = connect().await?;
    let gid: i64 = guildid.into();

    match sqlx::query("SELECT prefix FROM prefix WHERE guild_id == ?1;")
        .bind(gid)
        .fetch_one(&mut conn)
        .await {
            Ok(r) => r.try_get(0).map_err(|e| BotError::DbError(e)),
            Err(sqlx::Error::RowNotFound) => Err(BotError::PrefixNotFound),
            Err(e) => Err(BotError::DbError(e))
     }
}
