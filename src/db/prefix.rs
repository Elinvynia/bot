use super::get_db;
use crate::data::error::BotError;
use serenity::{model::prelude::*, prelude::*};
use sqlx::prelude::*;

pub async fn get_prefix(guildid: GuildId, ctx: &Context) -> Result<String, BotError> {
    let mut conn = get_db(ctx).await?;
    let gid: i64 = guildid.into();
    let prefix = sqlx::query("SELECT prefix FROM prefix WHERE guild_id == ?1;")
        .bind(gid)
        .fetch(&mut conn)
        .next()
        .await?
        .unwrap()
        .try_get(0)?;
    Ok(prefix)
}
