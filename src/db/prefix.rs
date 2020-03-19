use super::get_db;
use crate::data::{cache::GuildPrefixes, error::BotError};
use serenity::{framework::standard::macros::hook, model::prelude::*, prelude::*};

#[hook]
pub async fn get_prefix(guildid: &GuildId, ctx: &Context) -> Result<String, BotError> {
    let conn = get_db()?;
    let prefix: String = tokio::task::block_in_place(|| {
        let mut statement = conn.prepare("SELECT prefix FROM prefix WHERE guild_id == ?1;")?;
        let mut rows = statement.query(&[&guildid.as_u64().to_string()])?;
        rows.next()?.unwrap().get(0)
    })?;
    let mut data = ctx.data.write().await;
    let prefixes = data.get_mut::<GuildPrefixes>().unwrap();
    prefixes.insert(guildid.clone(), prefix.clone());
    Ok(prefix)
}
