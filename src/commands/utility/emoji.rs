use crate::prelude::*;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};

#[command]
#[only_in(guilds)]
#[num_args(1)]
#[description("Sends an emoji as an enlargened image.")]
#[usage("emoji <emoji>")]
#[example("emoji :HeyGuys:")]
async fn emoji(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let gid = msg.guild_id.ok_or(BotError::NoneError)?;

    let emoji = none_return_ok!(parse_reaction(&args.single::<String>()?, &gid, &ctx).await);
    let url = emoji.url();

    msg.channel_id
        .send_message(&ctx, |msg| {
            msg.add_file(url.as_str());
            msg
        })
        .await?;

    Ok(())
}
