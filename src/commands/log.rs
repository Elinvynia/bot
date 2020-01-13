use diesel::dsl::{insert_into, update};
use diesel::prelude::*;

use crate::functions::*;
use crate::data::*;
use crate::db::models::LogChannel;
use crate::db::schema::log_channels::dsl::*;

use serenity::framework::standard::{macros::command, CommandResult, Args};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[only_in(guilds)]
#[owners_only]
#[min_args(0)]
#[max_args(2)]
fn log(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read();

    let pool = data.get::<DatabaseConnection>().unwrap();
    let conn = pool.get().unwrap();

    let cid = msg.channel_id.0 as i64;
    let gid = msg.guild_id.unwrap().0 as i64;

    if args.len() == 2 {
        let log_channel = get_log_channel(&ctx, &msg.guild_id.unwrap());
        if log_channel == ChannelId(0) {
            return Ok(())
        }
        let mut previous_type = get_log_type(&ctx, &msg.guild_id.unwrap());
        let on_off = args.single::<String>().unwrap();
        let log_kind = args.single::<String>().unwrap();
        let message: String;

        match &on_off[..] {
            "enable" => {
                match &log_kind[..] {
                    "all" => {
                        previous_type |= LogType::All as i64;
                        message = "All messages will now be logged!".to_string();
                    },
                    "delete" => {
                        previous_type |= LogType::MessageDeleted as i64;
                        message = "Deleted messages will now be logged!".to_string();
                    },
                    "edit" => {
                        previous_type |= LogType::MessageEdited as i64;
                        message = "Edited messages will now be logged!".to_string();
                        },
                    "join" => {
                        previous_type |= LogType::UserJoined as i64;
                        message = "Join messages will now be logged!".to_string();
                        },
                    "leave" => {
                        previous_type |= LogType::UserLeft as i64;
                        message = "Leave messages will now be logged!".to_string();
                        },
                    _ => return Ok(())
                };
            },
            "disable" => {
                match &log_kind[..] {
                    "all" => {
                        previous_type |= LogType::All as i64;
                        message = "No messages will be logged anymore!".to_string();
                    },
                    "delete" => {
                        previous_type &= !(LogType::MessageDeleted as i64);
                        message = "Deleted messages will no longer be logged!".to_string();
                    },
                    "edit" => {
                        previous_type &= !(LogType::MessageEdited as i64);
                        message = "Edited messages will no longer be logged!".to_string();
                        },
                    "join" => {
                        previous_type &= !(LogType::UserJoined as i64);
                        message = "Join messages will no longer be logged!".to_string();
                        },
                    "leave" => {
                        previous_type &= !(LogType::UserLeft as i64);
                        message = "Leave messages will no longer be logged!".to_string();
                        },
                    _ => return Ok(())
                };
            },
            _ => return Ok(())
        }
        
        let _ = update(log_channels).filter(guild_id.eq(&gid)).set(log_type.eq(previous_type)).execute(&conn);
        let _ = log_channel.say(&ctx.http, message);
        return Ok(())
    }
    
    match log_channels
        .filter(guild_id.eq(&gid))
        .load::<LogChannel>(&conn)
    {
        Ok(arr) if !arr.is_empty() => {
            match update(log_channels)
                .filter(guild_id.eq(&gid))
                .set((channel_id.eq(&cid), guild_id.eq(&gid)))
                .execute(&conn)
            {
                Ok(_) => {
                    let _ = msg.channel_id.say(&ctx.http, "Log channel updated!");
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
            match insert_into(log_channels)
                .values((channel_id.eq(&cid), guild_id.eq(&gid), log_type.eq(LogType::All as i64)))
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
