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
    let guild_id = msg.guild_id.ok_or(anyhow!("Guild ID not found."))?;

    let banned_arg: String = error_return_ok!(args.single());
    let banned_id = none_return_ok!(parse_user(&banned_arg, Some(&guild_id), Some(&ctx)).await);

    let banned = banned_id.to_user(ctx).await?;
    let reason = format!("Eli Bot | {}", args.single::<String>().unwrap_or_else(|_| "".into()));
    let channel = banned.create_dm_channel(&ctx).await?;
    let guild = msg.guild(&ctx.cache).await.ok_or(anyhow!("Guild not found in cache."))?;

    guild_id.ban_with_reason(&ctx, banned, 0, &reason).await?;

    let banned_message = format!("You have been banned from {}\nReason: {}", guild.name, &reason);
    channel.say(&ctx, banned_message).await?;

    Ok(())
}
