use crate::util::parse_user;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};

#[command]
#[only_in(guilds)]
#[required_permissions(KICK_MEMBERS)]
#[min_args(1)]
#[max_args(2)]
fn kick(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let kicked_id = parse_user(
        &args.quoted().current().unwrap().to_string(),
        msg.guild_id.as_ref(),
        Some(&ctx),
    )
    .ok_or("arg passed isn't a valid user mention")?;
    let kicked = kicked_id.to_user(&ctx)?;

    args.advance();
    let arg_reason = args.current().unwrap_or("");

    kicked.create_dm_channel(&ctx).unwrap().say(
        &ctx.http,
        format!(
            "You have been kicked from {}\nReason: {}",
            msg.guild(&ctx.cache).unwrap().read().name,
            &arg_reason
        ),
    )?;
    msg.guild_id.unwrap().kick(&ctx.http, kicked)?;

    Ok(())
}
