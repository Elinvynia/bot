use crate::util::parse_user;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};

#[command]
#[only_in(guilds)]
#[required_permissions(BAN_MEMBERS)]
#[min_args(1)]
#[max_args(2)]
fn ban(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let banned_id = match parse_user(&args.current().unwrap().to_string(), Some(&msg)) {
        Some(x) => x,
        None => return Ok(()),
    };
    let banned = banned_id.to_user(&ctx)?;

    args.advance();
    let arg_reason = args.current().unwrap_or("");
    let reason = format!("Eli Bot | {}", arg_reason);

    let _ = banned.create_dm_channel(&ctx).unwrap().say(
        &ctx.http,
        format!(
            "You have been banned from {}\nReason: {}",
            msg.guild(&ctx.cache).unwrap().read().name,
            &arg_reason
        ),
    );
    let _ = msg.guild_id.unwrap().ban(&ctx.http, banned, &(0, reason));

    Ok(())
}
