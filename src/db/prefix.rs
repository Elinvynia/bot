use super::get_db;
use crate::data::{cache::GuildPrefixes, error::BotError};
use serenity::{model::prelude::*, prelude::*};

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
