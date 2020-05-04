use crate::utils::parse::parse_user;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};

#[command]
#[min_args(0)]
#[max_args(1)]
async fn user(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let user_id = if args.len() == 1 {
        match parse_user(
            &args.quoted().current().unwrap().to_string(),
            msg.guild_id.as_ref(),
            Some(&ctx),
        )
        .await
        {
            Some(i) => i,
            None => msg.author.id,
        }
    } else {
        msg.author.id
    };

    let user = user_id.to_user(ctx).await?;

    msg.channel_id
        .say(
            &ctx,
            format!("User found!\nTag: {}\nID: {}", user.tag(), user.id),
        )
        .await?;

    Ok(())
}
