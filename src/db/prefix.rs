use super::get_db;
use crate::data::error::BotError;
use serenity::{model::prelude::*, prelude::*};

pub fn get_prefix(guildid: GuildId, _: &Context) -> Result<String, BotError> {
    let conn = get_db()?;
    let mut statement = conn.prepare("SELECT prefix FROM prefix WHERE guild_id == ?1;")?;
    let mut rows = statement.query(&[&guildid.as_u64().to_string()])?;
    let prefix = rows.next()?.unwrap().get(0)?;
    Ok(prefix)
}
