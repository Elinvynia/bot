use serenity::{prelude::*, model::prelude::*};
use crate::data::{error::BotError, cache::GuildPrefixes};
use super::get_db;

pub fn get_prefix(guildid: &GuildId, ctx: &Context) -> Result<String, BotError> {
    let conn = get_db()?;
    let mut statement = conn.prepare("SELECT prefix FROM prefix WHERE guild_id == ?1;")?;
    let mut rows = statement.query(&[&guildid.as_u64().to_string()])?;
    let prefix: String = rows.next()?.ok_or("Guild not found.".to_string())?.get(0)?;
    {
        let mut data = ctx.data.write();
        let prefixes = data.get_mut::<GuildPrefixes>().unwrap();
        prefixes.insert(guildid.clone(), prefix.clone());
    }
    Ok(prefix)
}
