use super::connect;
use crate::data::error::BotError;
use serenity::{model::prelude::*, prelude::*};
use sqlx::prelude::*;

pub async fn get_prefix(guildid: GuildId, _ctxx: &Context) -> Result<String, BotError> {
    let mut conn = connect().await?;
    let gid: i64 = guildid.into();

    if let Some(x) = sqlx::query("SELECT prefix FROM prefix WHERE guild_id == ?1;")
        .bind(gid)
        .fetch(&mut conn)
        .next()
        .await?
    {
        let y = x.try_get(0)?;
        Ok(y)
    } else {
        Err(BotError::CustomError("".to_string()))
    }
}
