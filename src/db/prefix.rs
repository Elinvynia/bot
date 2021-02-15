use crate::prelude::*;
use serenity::model::prelude::*;

pub async fn get_prefix(guildid: GuildId) -> Result<String> {
    let conn = connect()?;
    let gid: i64 = guildid.into();

    let mut s = conn.prepare("SELECT prefix FROM prefix WHERE guild_id == ?1;")?;
    let r = s.query_row(params![gid], |r| r.get(0))?;
    Ok(r)
}
