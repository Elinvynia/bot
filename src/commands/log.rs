use diesel::dsl::{insert_into, update};
use diesel::prelude::*;

use crate::data::*;
use crate::db::schema::log_channels::dsl::*;

use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[only_in(guilds)]
fn log(ctx: &mut Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read();

    let pool = data.get::<DatabaseConnection>().unwrap();
    let conn = pool.get().unwrap();

    let cid = msg.channel_id.0 as i64;
    let gid = msg.guild_id.unwrap().0 as i64;

    match log_channels.filter(guild_id.eq(&gid)).execute(&conn) {
        Ok(0) => {
            match insert_into(log_channels)
                .values((channel_id.eq(&cid), guild_id.eq(&gid)))
                .execute(&conn)
            {
                Ok(_) => {
                    let _ = msg.channel_id.say(&ctx.http, "Log channel set!");
                }
                Err(e) => {
                    let _ = msg.channel_id.say(
                        &ctx.http,
                        "Failed to set log channel, check the error logs!",
                    );
                    println!("{:?}", e);
                }
            }
        }
        Ok(_) => {
            match update(log_channels)
                .filter(guild_id.eq(&gid))
                .set((channel_id.eq(&cid), guild_id.eq(&gid)))
                .execute(&conn)
            {
                Ok(_) => {
                    let _ = msg.channel_id.say(&ctx.http, "Log channel set!");
                }
                Err(e) => {
                    let _ = msg.channel_id.say(
                        &ctx.http,
                        "Failed to set log channel, check the error logs!",
                    );
                    println!("{:?}", e);
                }
            }
        }
        Err(e) => {
            let _ = msg.channel_id.say(
                &ctx.http,
                "Failed to set log channel, check the error logs!",
            );
            println!("{:?}", e);
        }
    }
    Ok(())
}
