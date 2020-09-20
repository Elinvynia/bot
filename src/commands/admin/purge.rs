use crate::prelude::*;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    futures::StreamExt,
    model::prelude::*,
    prelude::*,
};

#[command]
#[only_in(guilds)]
#[required_permissions(MANAGE_MESSAGES)]
#[num_args(1)]
#[description("Purges a set amount of messages from a channel.")]
#[usage("purge <num>")]
#[example("purge 20")]
async fn purge(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let amount: u64 = error_return_ok!(args.single());
    let guild_id = msg.guild_id.ok_or(BotError::NoneError)?;

    let purge_message = format!(
        "Are you sure you want to purge {} messages?\n Type \"yes\" to confirm.",
        amount
    );
    msg.channel_id.say(&ctx, purge_message).await?;
    msg.channel_id
        .await_reply(&ctx)
        .timeout(std::time::Duration::from_secs(15))
        .author_id(msg.author.id)
        .channel_id(msg.channel_id)
        .guild_id(guild_id)
        .filter(|msg| msg.content == "yes");

    let mut iter = msg.channel_id.messages_iter(&ctx.http).boxed();
    for _ in 0..amount {
        if let Some(Ok(message)) = iter.next().await {
            message.delete(&ctx).await?;
        };
    }

    Ok(())
}
