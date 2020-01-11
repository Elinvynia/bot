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

    match log_channels
        .select(channel_id)
        .filter(guild_id.eq(&gid))
        .load::<i64>(&conn)
    {
        Ok(arr) if !arr.is_empty() => true,
        Ok(_) => false,
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

    match log_channels
        .select(channel_id)
        .filter(guild_id.eq(gid))
        .load::<i64>(&conn)
    {
        Ok(l) => ChannelId(l[0] as u64),
        Err(e) => {
            println!("{:?}", e);
            ChannelId(0)
        }
    }
}


pub fn get_log_type(ctx: &Context, guildid: &GuildId) -> i64 {
    let data = ctx.data.read();

    let pool = data.get::<DatabaseConnection>().unwrap();
    let conn = pool.get().unwrap();

    let gid = guildid.0 as i64;

    match log_channels
        .select(log_type)
        .filter(guild_id.eq(gid))
        .load::<i64>(&conn)
    {
        Ok(vec) => vec[0],
        Err(e) => {
            println!("{:?}", e);
            0
        }
    }
}
