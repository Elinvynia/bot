use diesel::sqlite::Sqlite;
use diesel::debug_query;
use diesel::prelude::*;

use crate::data::*;
use crate::db::schema::log_channels::dsl::*;

use serenity::model::prelude::*;
use serenity::prelude::*;

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
                format!("{} from {}", &message.content, &message.author),
            );
    }
}

pub fn check_log_channel(ctx: &Context, guildid: &GuildId) -> bool {
    let data = ctx.data.read();

    let pool = data.get::<DatabaseConnection>().unwrap();
    let conn = pool.get().unwrap();

    let gid = guildid.0 as i64;

    println!("{:?}", debug_query::<Sqlite, _>(&log_channels.select(channel_id).filter(guild_id.eq(&gid))));
    println!("{:?}", log_channels.select(channel_id).filter(guild_id.eq(&gid)).execute(&conn));

    match log_channels.select(channel_id).filter(guild_id.eq(&gid)).execute(&conn) {
        Ok(0) => {
            println!("no log channel");
            false
        }
        Ok(v) => {
            println!("log channel{:?}", v);
            true
        }
        Err(e) => {
            println!("{:?}", e);
            false
        }
    }
}

pub fn get_log_channel(ctx: &Context, guildid: &GuildId) -> ChannelId {
    let data = ctx.data.read();

    let pool = data.get::<DatabaseConnection>().unwrap();
    let conn = pool.get().unwrap();

    let gid = guildid.0 as i64;

    match log_channels.select(channel_id).filter(guild_id.eq(gid)).execute(&conn) {
        Ok(cid) => ChannelId(cid as u64),
        Err(e) => {
            println!("{:?}", e);
            ChannelId(0)
        }
    }
}
