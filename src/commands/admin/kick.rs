use crate::prelude::*;
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
    args.quoted();

    let kicked_id = match parse_user(&args.single::<String>()?, msg.guild_id.as_ref(), Some(&ctx)).await {
        Some(id) => id,
        None => return Ok(()),
    };

    let kicked = kicked_id.to_user(ctx).await?;
    let reason = format!("Eli Bot | {}", args.single::<String>().unwrap_or_else(|_| "".into()));
    let channel = kicked.create_dm_channel(&ctx).await?;
    let guild = msg.guild(&ctx.cache).await.ok_or(BotError::NoneError)?;

    msg.guild_id
        .ok_or(BotError::NoneError)?
        .kick_with_reason(&ctx, kicked, &reason)
        .await?;

    let kicked_message = format!("You have been kicked from {}\nReason: {}", &guild.name, &reason);
    channel.say(&ctx, kicked_message).await?;

    Ok(())
}
