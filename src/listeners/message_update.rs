use crate::data::{cache::BotId, db::LogType};
use crate::db::log::{get_log_channel, get_log_type};
use log::error;
use serenity::{model::prelude::*, prelude::*};

pub async fn message_update(
    ctx: Context,
    old: Option<Message>,
    new: Option<Message>,
    _: MessageUpdateEvent,
) {
    if old.is_none() || new.is_none() {
        return;
    }

    let old_m = old.unwrap();
    let new_m = new.unwrap();
    let guildid = new_m.guild_id.unwrap();

    let log_channel = match get_log_channel(&ctx, guildid).await {
        Ok(l) => l,
        Err(_) => {
            return;
        }
    };

    let data = ctx.data.read();
    if new_m.author.id == *data.await.get::<BotId>().unwrap() {
        return;
    }

    if new_m.guild_id.is_none() {
        return;
    }

    let log_type = match get_log_type(&ctx, guildid).await {
        Ok(l) => l,
        Err(_) => {
            return;
        }
    };

    if log_type & LogType::MessageEdited as i64 != LogType::MessageEdited as i64 {
        return;
    }

    if let Err(e) = log_channel
        .say(
            &ctx.http,
            format!(
                "Message by {} updated in channel {} from:\n{}\nTo:\n{}",
                new_m.author,
                new_m.channel(&ctx.cache).await.unwrap(),
                old_m.content,
                new_m.content
            ),
        )
        .await
    {
        error!("{:?}", e);
    }
}
