use crate::data::{cache::BotId, db::LogType};
use crate::db::log::{check_log_type, log_channel_say};
use serenity::{model::prelude::*, prelude::*};

pub async fn message_update(ctx: Context, old: Option<Message>, new: Option<Message>, _: MessageUpdateEvent) {
    let old_m = match old {
        Some(m) => m,
        None => return,
    };

    let new_m = match new {
        Some(m) => m,
        None => return,
    };

    let guildid = match new_m.guild_id {
        Some(gid) => gid,
        None => return,
    };

    let data = ctx.data.read().await;
    let botid = match data.get::<BotId>() {
        Some(bid) => bid,
        None => return,
    };

    if &new_m.author.id == botid {
        return;
    }

    if let Ok(user) = new_m.author.id.to_user(&ctx).await {
        if user.bot {
            return;
        };
    };

    if check_log_type(LogType::MessageEdited, guildid).await.is_err() {
        return;
    }

    let channel = match new_m.channel(&ctx.cache).await {
        Some(c) => c,
        None => return,
    };

    let _ = log_channel_say(
        &ctx,
        guildid,
        &format!(
            "Message by {} updated in channel {} from:\n{}\nTo:\n{}",
            new_m.author, channel, old_m.content, new_m.content
        ),
    )
    .await;
}
