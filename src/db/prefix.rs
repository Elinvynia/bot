use crate::prelude::*;
use serenity::model::prelude::*;

pub async fn get_prefix(guildid: GuildId) -> Result<String> {
    let mut conn = connect().await?;
    let gid = guildid.to_string();

    let r = sqlx::query!("SELECT prefix FROM prefix WHERE guild_id == ?1;", gid)
        .fetch_one(&mut conn)
        .await?;
    Ok(r.prefix)
}
