use crate::{
    data::db::LogType,
    db::{
        get_db,
        log::{get_log_channel, get_log_type},
    },
};
use log::error;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};

#[command]
#[only_in(guilds)]
#[owners_only]
#[min_args(0)]
#[max_args(2)]
async fn log(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();
    let cid = msg.channel_id.0 as i64;
    let gid = guild_id.0 as i64;
    let conn = get_db()?;

    if args.is_empty() {
        if get_log_channel(guild_id).is_ok() {
            let _ = conn.execute(
                "UPDATE log SET channel_id = ?1 WHERE guild_id == ?2;",
                &[&cid.to_string(), &gid.to_string()],
            )?;
            let log_channel = get_log_channel(guild_id)?;
            log_channel.say(&ctx.http, "Log channel updated!").await?;
            return Ok(());
        } else {
            let _ = conn.execute(
                "INSERT INTO log (guild_id, channel_id, log_type) values (?1, ?2, ?3)",
                &[
                    &gid.to_string(),
                    &cid.to_string(),
                    &(LogType::All as u64).to_string(),
                ],
            )?;
            msg.channel_id.say(&ctx.http, "Log channel set!").await?;
            return Ok(());
        };
    }

    if args.len() == 2 {
        let log_channel = get_log_channel(guild_id)?;

        let mut log_type = match get_log_type(guild_id) {
            Ok(l) => l,
            Err(e) => {
                error!("{:?}", e);
                return Ok(());
            }
        };
        let on_off = args.single::<String>()?;
        let log_kind = args.single::<String>()?;
        let message: String;

        match &on_off[..] {
            "enable" => {
                match &log_kind[..] {
                    "all" => {
                        log_type |= LogType::All as i64;
                        message = "All messages will now be logged!".to_string();
                    }
                    "delete" => {
                        log_type |= LogType::MessageDeleted as i64;
                        message = "Deleted messages will now be logged!".to_string();
                    }
                    "edit" => {
                        log_type |= LogType::MessageEdited as i64;
                        message = "Edited messages will now be logged!".to_string();
                    }
                    "join" => {
                        log_type |= LogType::UserJoined as i64;
                        message = "Join messages will now be logged!".to_string();
                    }
                    "leave" => {
                        log_type |= LogType::UserLeft as i64;
                        message = "Leave messages will now be logged!".to_string();
                    }
                    _ => return Ok(()),
                };
            }
            "disable" => {
                match &log_kind[..] {
                    "all" => {
                        log_type &= !(LogType::All as i64);
                        message = "No messages will be logged anymore!".to_string();
                    }
                    "delete" => {
                        log_type &= !(LogType::MessageDeleted as i64);
                        message = "Deleted messages will no longer be logged!".to_string();
                    }
                    "edit" => {
                        log_type &= !(LogType::MessageEdited as i64);
                        message = "Edited messages will no longer be logged!".to_string();
                    }
                    "join" => {
                        log_type &= !(LogType::UserJoined as i64);
                        message = "Join messages will no longer be logged!".to_string();
                    }
                    "leave" => {
                        log_type &= !(LogType::UserLeft as i64);
                        message = "Leave messages will no longer be logged!".to_string();
                    }
                    _ => return Ok(()),
                };
            }
            _ => return Ok(()),
        }

        let _ = conn.execute(
            "UPDATE log SET log_type = ?2 WHERE guild_id = ?1;",
            &[&gid.to_string(), &log_type.to_string()],
        )?;
        log_channel.say(&ctx.http, message).await?;
        return Ok(());
    }

    return Ok(());
}
