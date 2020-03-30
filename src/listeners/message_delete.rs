use crate::data::{cache::BotId, db::LogType};
use crate::db::log::{get_log_channel, get_log_type};
use serenity::{model::prelude::*, prelude::*};

pub async fn message_delete(ctx: Context, channel: ChannelId, deleted_message_id: MessageId) {
    let guildid = channel
        .to_channel(&ctx)
        .await
        .unwrap()
        .guild()
        .unwrap()
        .read()
        .await
        .guild_id;

    let log_channel = match get_log_channel(&ctx, guildid).await {
        Ok(l) => l,
        Err(_) => {
            return;
        }
    };

    let log_type = match get_log_type(&ctx, guildid).await {
        Ok(l) => l,
        Err(_) => {
            return;
        }
    };

    if log_type & LogType::MessageDeleted as i64 != LogType::MessageDeleted as i64 {
        return;
    }

    if let Some(x) = ctx
        .cache
        .read()
        .await
        .message(&channel, &deleted_message_id)
    {
        let data = ctx.data.read();
        if x.author.id == *data.await.get::<BotId>().unwrap() {
            return;
        }
        let _ = log_channel
            .say(
                &ctx.http,
                format!(
                    "Message by {} deleted in channel {}:\n{}",
                    x.author,
                    x.channel(&ctx.cache).await.unwrap(),
                    x.content
                ),
            )
            .await;
    }
}
