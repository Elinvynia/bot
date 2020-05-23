use crate::{
    data::db::LogType,
    db::{
        connect,
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
async fn log(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();
    let cid = msg.channel_id.0 as i64;
    let gid = guild_id.0 as i64;
    let mut conn = connect().await?;

    if args.is_empty() {
        if get_log_channel(&ctx, guild_id).await.is_ok() {
            sqlx::query("UPDATE log SET channel_id = ?1 WHERE guild_id == ?2;")
                .bind(&cid.to_string())
                .bind(&gid.to_string())
                .execute(&mut conn)
                .await?;
            let log_channel = get_log_channel(&ctx, guild_id).await?;
            log_channel.say(&ctx.http, "Log channel updated!").await?;
            return Ok(());
        } else {
            sqlx::query("INSERT INTO log (guild_id, channel_id, log_type) values (?1, ?2, ?3)")
                    .bind(&gid.to_string())
                    .bind(&cid.to_string())
                    .bind(&(LogType::All as u64).to_string())
                    .execute(&mut conn)
                    .await?;
            msg.channel_id.say(&ctx.http, "Log channel set!").await?;
            return Ok(());
        };
    }

    if args.len() == 2 {
        let log_channel = get_log_channel(&ctx, guild_id).await?;

        let mut log_type = match get_log_type(&ctx, guild_id).await {
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
        sqlx::query("UPDATE log SET log_type = ?1 WHERE guild_id = ?2;")
            .bind(&log_type.to_string())
            .bind(&gid.to_string())
            .execute(&mut conn)
            .await?;
        log_channel.say(&ctx.http, message).await?;
        return Ok(());
    }

    return Ok(());
}
