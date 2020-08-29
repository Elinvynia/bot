use crate::prelude::*;
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
#[description("Bans a user from the server.")]
#[usage("ban <person> <optional: reason>")]
#[example("ban @Elinvynia \"Abusive language\"")]
async fn ban(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    args.quoted();

    let banned_id = match parse_user(&args.single::<String>()?, msg.guild_id.as_ref(), Some(&ctx)).await {
        Some(id) => id,
        None => return Ok(()),
    };

    let banned = banned_id.to_user(ctx).await?;
    let reason = format!("Eli Bot | {}", args.single::<String>().unwrap_or_else(|_| "".into()));
    let channel = banned.create_dm_channel(&ctx).await?;
    let guild = msg.guild(&ctx.cache).await.ok_or(BotError::NoneError)?;

    msg.guild_id
        .ok_or(BotError::NoneError)?
        .ban_with_reason(&ctx, banned, 0, &reason)
        .await?;

    let banned_message = format!("You have been banned from {}\nReason: {}", guild.name, &reason);
    channel.say(&ctx, banned_message).await?;

    Ok(())
}
