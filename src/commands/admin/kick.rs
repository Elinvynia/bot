use crate::{data::error::BotError, utils::parse::parse_user};
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
#[description("Kicks a user from the server.")]
#[usage("kick <person> <optional: reason>")]
#[example("kick @Elinvynia \"Abusive language\"")]
async fn kick(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let kicked_id = parse_user(
        &args.quoted().current().ok_or(BotError::NoneError)?.to_string(),
        msg.guild_id.as_ref(),
        Some(&ctx),
    )
    .await
    .ok_or("arg passed isn't a valid user mention")?;
    let kicked = kicked_id.to_user(ctx).await?;

    args.advance();
    let arg_reason = args.current().unwrap_or("");
    let reason = format!("Eli Bot | {}", &arg_reason);

    let channel = kicked.create_dm_channel(&ctx).await?;

    let guild = msg.guild(&ctx.cache).await.ok_or(BotError::NoneError)?;
    let guild_name = &guild.name;

    channel
        .say(
            &ctx.http,
            format!("You have been kicked from {}\nReason: {}", &guild_name, &arg_reason),
        )
        .await?;

    msg.guild_id
        .ok_or(BotError::NoneError)?
        .kick_with_reason(&ctx.http, kicked, &reason)
        .await?;

    Ok(())
}
