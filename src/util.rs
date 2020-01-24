use crate::data::*;
use crate::db::*;
use serenity::{model::prelude::*, prelude::*};
use std::error::Error;

pub fn log_dm(ctx: &mut Context, message: &Message) {
    if message.guild_id.is_some() {
        return;
    }

    let data = ctx.data.read();

    if &message.author.id == data.get::<BotId>().unwrap() {
        return;
    }

    let owners = data.get::<BotOwners>().unwrap();
    for x in owners.iter() {
        if &message.author.id == x {
            continue;
        }
        let _ = x
            .to_user(&ctx)
            .unwrap()
            .create_dm_channel(&ctx)
            .unwrap()
            .say(
                &ctx.http,
                format!("DM from {}:\n{}", &message.author, &message.content),
            );
    }
}

pub fn get_log_channel(guildid: &GuildId) -> Result<ChannelId, Box<dyn Error>> {
    let conn = get_db()?;

    let mut statement = conn.prepare("SELECT channel_id FROM log WHERE guild_id == ?1;")?;
    let mut rows = statement.query(&[&guildid.as_u64().to_string()])?;
    let result: String = rows.next()?.ok_or("Guild not found.")?.get(0)?;
    let cid: u64 = result.parse()?;

    Ok(ChannelId(cid))
}

pub fn get_log_type(guildid: &GuildId) -> Result<i64, Box<dyn Error>> {
    let conn = get_db()?;

    let mut statement = conn.prepare("SELECT log_type FROM log WHERE guild_id == ?1;")?;
    let mut rows = statement.query(&[&guildid.as_u64().to_string()])?;
    let result: String = rows.next()?.ok_or("Guild not found.")?.get(0)?;
    let log_type: i64 = result.parse()?;

    Ok(log_type)
}

pub fn get_prefix(guildid: &GuildId) -> Result<String, Box<dyn Error>> {
    let conn = get_db()?;
    let mut statement = conn.prepare("SELECT prefix FROM prefix WHERE guild_id == ?1;")?;
    let mut rows = statement.query(&[&guildid.as_u64().to_string()])?;
    Ok(rows.next()?.ok_or("Guild not found.")?.get(0)?)
}

pub fn get_user_score(guildid: &GuildId, userid: &UserId) -> Result<i64, Box<dyn Error>> {
    let conn = get_db()?;
    let mut statement = conn.prepare("SELECT points FROM leaderboard WHERE guild_id == ?1 AND user_id == ?2;")?;
    let mut rows = statement.query(&[&guildid.as_u64().to_string(), &userid.as_u64().to_string()])?;
    Ok(rows.next()?.ok_or("No record yet.")?.get(0)?)
}
