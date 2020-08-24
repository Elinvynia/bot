use crate::utils::parse::parse_reaction;
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
    let gid = msg.guild_id.unwrap();

    let emoji = match parse_reaction(&args.single::<String>().unwrap(), &gid, &ctx).await {
        Some(e) => e,
        None => return Ok(()),
    };

    let url = emoji.url();

    msg.channel_id
        .send_message(&ctx, |msg| {
            msg.add_file(url.as_str());
            msg
        })
        .await?;

    Ok(())
}
