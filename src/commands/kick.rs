use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
    utils::parse_username,
};

#[command]
#[only_in(guilds)]
#[required_permissions(KICK_MEMBERS)]
#[min_args(1)]
#[max_args(2)]
fn kick(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let kicked_id = parse_username(args.current().ok_or("no args passed")?)
        .ok_or("arg passed isn't a valid user mention")?;
    let kicked = UserId(kicked_id).to_user(&ctx)?;

    args.advance();
    let arg_reason = args.current().unwrap_or("");

    let _ = kicked.create_dm_channel(&ctx).unwrap().say(
        &ctx.http,
        format!(
            "You have been kicked from {}\nReason: {}",
            msg.guild(&ctx.cache).unwrap().read().name,
            &arg_reason
        ),
    );
    let _ = msg.guild_id.unwrap().kick(&ctx.http, kicked);

    Ok(())
}
