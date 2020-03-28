use super::get_db;
use crate::data::error::BotError;
use serenity::model::prelude::*;

pub fn get_log_channel(guildid: GuildId) -> Result<ChannelId, BotError> {
    let conn = match get_db() {
        Ok(c) => c,
        Err(e) => return Err(e),
    };

    let mut statement = conn.prepare("SELECT channel_id FROM log WHERE guild_id == ?1;")?;
    let mut rows = statement.query(&[&guildid.as_u64().to_string()])?;
    let result: String = rows
        .next()?
        .ok_or_else(|| "Guild not found.".to_string())?
        .get(0)?;
    let cid: u64 = result.parse()?;

    Ok(ChannelId(cid))
}

pub fn get_log_type(guildid: GuildId) -> Result<i64, BotError> {
    let conn = get_db()?;

    let mut statement = conn.prepare("SELECT log_type FROM log WHERE guild_id == ?1;")?;
    let mut rows = statement.query(&[&guildid.as_u64().to_string()])?;
    let result: String = rows
        .next()?
        .ok_or_else(|| "Guild not found.".to_string())?
        .get(0)?;
    let log_type: i64 = result.parse()?;

    Ok(log_type)
}
