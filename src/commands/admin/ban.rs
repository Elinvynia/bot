use crate::utils::parse::parse_user;
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
async fn ban(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let banned_id = parse_user(
        &args.quoted().current().unwrap().to_string(),
        msg.guild_id.as_ref(),
        Some(&ctx),
    )
    .await
    .ok_or("Arg passed isn't a valid user mention.")?;
    let banned = banned_id.to_user(&ctx).await?;

    args.advance();
    let arg_reason = args.current().unwrap_or("");
    let reason = format!("Eli Bot | {}", arg_reason);

    let channel = banned.create_dm_channel(&ctx).await.unwrap();

    channel
        .say(
            &ctx.http,
            format!(
                "You have been banned from {}\nReason: {}",
                msg.guild(&ctx.cache).await.unwrap().read().await.name,
                &arg_reason
            ),
        )
        .await?;
    msg.guild_id
        .unwrap()
        .ban(&ctx.http, banned, &(0, reason))
        .await?;

    Ok(())
}
