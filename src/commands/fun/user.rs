use crate::utils::parse::parse_user;
use serenity::{
    framework::standard::{macros::command, Args, CommandError, CommandResult},
    model::prelude::*,
    prelude::*,
};

#[command]
#[min_args(0)]
#[max_args(1)]
async fn user(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let user_id;
    if args.len() == 1 {
        user_id = match parse_user(
            &args.quoted().current().unwrap().to_string(),
            msg.guild_id.as_ref(),
            Some(&ctx),
        )
        .await
        {
            Some(i) => i,
            None => return Err(CommandError("No user found".to_string())),
        };
    } else {
        user_id = msg.author.id;
    }

    let user = user_id.to_user(&ctx).await?;

    msg.channel_id
        .say(
            &ctx,
            format!("User found!\nTag: {}\nID: {}", user.tag(), user.id),
        )
        .await?;

    Ok(())
}
