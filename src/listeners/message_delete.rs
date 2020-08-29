use crate::data::{cache::BotId, db::LogType};
use crate::db::log::{check_log_type, log_channel_say};
use serenity::{model::prelude::*, prelude::*};

pub async fn message_delete(ctx: Context, channel: ChannelId, deleted_message_id: MessageId) {
    let channel: Channel = match channel.to_channel(&ctx).await {
        Ok(c) => c,
        Err(_) => return,
    };

    let guildid = match channel.clone() {
        Channel::Guild(g) => g.guild_id,
        _ => return,
    };

    if check_log_type(LogType::MessageDeleted, guildid).await.is_err() {
        return;
    }

    let message = match ctx.cache.message(&channel.id(), &deleted_message_id).await {
        Some(m) => m,
        None => return,
    };

    let data = ctx.data.read().await;
    let botid = match data.get::<BotId>() {
        Some(id) => id,
        None => return,
    };

    if &message.author.id == botid {
        return;
    };

    let channel = match message.channel(&ctx.cache).await {
        Some(c) => c,
        None => return,
    };

    let _ = log_channel_say(
        &ctx,
        guildid,
        &format!(
            "Message by {} deleted in channel {}:\n{}",
            message.author, channel, message.content
        ),
    )
    .await;
}
