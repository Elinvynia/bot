use crate::prelude::*;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
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
    let mut amount = args.single::<i64>()?;
    let guild_id = msg.guild_id.ok_or(BotError::NoneError)?;

    msg.channel_id.say(&ctx, format!("Are you sure you want to purge {} messages?\n Type \"yes\" to confirm.", amount)).await?;
    msg.channel_id.await_reply(&ctx)
        .timeout(std::time::Duration::from_secs(15))
        .author_id(msg.author.id)
        .channel_id(msg.channel_id)
        .guild_id(guild_id)
        .filter(|msg| msg.content == "yes");

    let mut messages: Vec<Message> = vec![];
    let mut last_id: Option<MessageId> = None;

    while amount > 100 {
        let mut msgs: Vec<Message> = vec![];
        if last_id.is_none() {
            msgs = msg.channel_id.messages(&ctx, |builder| {
                builder.before(msg.id).limit(amount as u64)
            }).await?;
        } else {
            let lastid = last_id.ok_or(BotError::NoneError)?;
            msgs = msg.channel_id.messages(&ctx, |builder| {
                builder.before(lastid).limit(amount as u64)
            }).await?;
        }

        last_id = Some(msgs.last().ok_or(BotError::NoneError)?.id);
        messages.append(&mut msgs);
        amount -= 100;
    };

    let messages = msg.channel_id.messages(&ctx, |builder| {
        builder.before(msg.id).limit(amount as u64)
    }).await?;

    for message in messages {
        message.delete(&ctx).await?;
    };

    Ok(())
}
