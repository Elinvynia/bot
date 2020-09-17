use crate::prelude::*;
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
        Some(msg) => {
            let data = ctx.data.read().await;
            let botid = match data.get::<BotId>() {
                Some(id) => id,
                None => return,
            };
            if &msg.author.id == botid {
                return;
            };
            let channel = match msg.channel(&ctx.cache).await {
                Some(c) => c,
                None => return,
            };
            let mut message = String::from("**Message Deleted**\n");
            message += &format!("ID: {}\n", msg.author.id);
            message += &format!("Tag: {}\n", msg.author.tag());
            message += &format!("Ping: {}\n", msg.author.mention());
            message += &format!("Channel: {}\n", channel);
            message += &format!("Content: \n{}\n", msg.content);
            message
        },
        None => {
            let mut message = String::from("**Message Deleted**\n");
            message += &format!("Message ID: {}\n", deleted_message_id);
            message
        },
    };

    let _ = log_channel_say(&ctx, guildid, &message).await;
}
