use serenity::prelude::*;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    utils::parse_username,
};

#[command]
#[only_in(guilds)]
#[required_permissions(BAN_MEMBERS)]
#[min_args(1)]
#[max_args(2)]
fn ban(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let banned_id = parse_username(args.current().ok_or("no args passed")?)
        .ok_or("arg passed isn't a valid user mention")?;
    let banned = UserId(banned_id).to_user(&ctx)?;

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
