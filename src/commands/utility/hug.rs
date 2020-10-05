use crate::prelude::*;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};

#[command]
#[only_in(guilds)]
#[num_args(1)]
#[description("Hugs another user.")]
#[usage("hug <user>")]
#[example("hug Elinvynia")]
async fn hug(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let gid = msg.guild_id.ok_or(BotError::NoneError)?;

    let user_id = none_return_ok!(parse_user(&args.single::<String>()?, Some(&gid), Some(&ctx)).await);
    let author = gid.member(&ctx, msg.author.id).await?;
    let member = gid.member(&ctx, user_id).await?;

    msg.channel_id
        .say(
            &ctx,
            format!("*{} hugs {}*", author.display_name(), member.display_name()),
        )
        .await?;

    Ok(())
}
