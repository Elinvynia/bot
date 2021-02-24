use crate::prelude::*;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};
use std::convert::TryInto;

#[command]
#[only_in(guilds)]
#[owners_only]
#[min_args(0)]
#[max_args(2)]
#[description = "Sets the log channel.  |  Toggles which type of event is logged in the log channel."]
#[usage = "log  |  log <enable|disable> <category>"]
#[example = "log enable join"]
async fn log(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut conn = connect().await?;

    let guild_id = msg.guild_id.ok_or_else(|| anyhow!("Guild ID not found"))?;
    let channel_id = msg.channel_id.to_string();

    if args.is_empty() {
        return log_channel(ctx, msg, conn, guild_id, channel_id).await;
    }

    if args.len() == 1 {
        return Ok(());
    }

    let log_channel = get_log_channel(guild_id).await?;
    let mut log_type = get_log_type(guild_id).await?;

    let on_off = args.single::<String>()?;
    let log_kind = args.single::<String>()?;
    let message: String;

    match &on_off[..] {
        "enable" => {
            let kind: LogType = log_kind.try_into()?;
            if kind == LogType::All {
                log_type = LogType::All as i64
            } else {
                log_type |= kind as i64;
            }
            message = format!("{} messages will now be logged!", kind.to_string());
        }
        "disable" => {
            let kind: LogType = log_kind.try_into()?;
            if kind == LogType::All {
                log_type = 0_i64;
            } else {
                log_type &= !(kind as i64);
            }
            message = format!("{} messages will no longer be logged!", kind.to_string());
        }
        _ => return Ok(()),
    }

    let gid = guild_id.to_string();
    sqlx::query!("UPDATE log SET log_type = ?1 WHERE guild_id = ?2;", log_type, gid)
        .execute(&mut conn)
        .await?;

    log_channel.say(&ctx, message).await?;

    Ok(())
}

async fn log_channel(
    ctx: &Context,
    msg: &Message,
    mut conn: SqliteConnection,
    guild_id: GuildId,
    cid: String,
) -> CommandResult {
    let gid = guild_id.to_string();
    if get_log_channel(guild_id).await.is_ok() {
        sqlx::query!("UPDATE log SET channel_id = ?1 WHERE guild_id == ?2;", cid, gid)
            .execute(&mut conn)
            .await?;
        let log_channel = get_log_channel(guild_id).await?;
        log_channel.say(&ctx, "Log channel updated!").await?;
    } else {
        sqlx::query!(
            "INSERT INTO log (guild_id, channel_id, log_type) values (?1, ?2, ?3)",
            gid,
            cid,
            LogType::All as i64
        )
        .execute(&mut conn)
        .await?;
        msg.channel_id.say(&ctx, "Log channel set!").await?;
    };
    Ok(())
}
