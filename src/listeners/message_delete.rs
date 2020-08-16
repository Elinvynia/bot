use crate::data::{cache::BotId, db::LogType};
use crate::db::log::{check_log_type, log_channel_say};
use serenity::{model::prelude::*, prelude::*};

pub async fn message_delete(ctx: Context, channel: ChannelId, deleted_message_id: MessageId) {
    let channel: Channel = channel.to_channel(&ctx).await.unwrap();
    let guildid = match channel.clone() {
        Channel::Guild(g) => g.guild_id,
        _ => return,
    };

    if check_log_type(LogType::MessageDeleted, guildid).await.is_err() {
        return;
    }

    if let Some(x) = ctx.cache.message(&channel.id(), &deleted_message_id).await {
        let data = ctx.data.read();
        if x.author.id == *data.await.get::<BotId>().unwrap() {
            return;
        }
        let _ = log_channel_say(
            &ctx,
            guildid,
            &format!(
                "Message by {} deleted in channel {}:\n{}",
                x.author,
                x.channel(&ctx.cache).await.unwrap(),
                x.content
            ),
        )
        .await;
    }
}
