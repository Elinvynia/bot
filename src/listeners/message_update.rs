use crate::data::{cache::BotId, db::LogType};
use crate::db::log::{check_log_type, log_channel_say};
use serenity::{model::prelude::*, prelude::*};

pub async fn message_update(ctx: Context, old: Option<Message>, new: Option<Message>, _: MessageUpdateEvent) {
    if old.is_none() || new.is_none() {
        return;
    }

    let old_m = old.unwrap();
    let new_m = new.unwrap();
    let guildid = new_m.guild_id.unwrap();
    let data = ctx.data.read();

    if &new_m.author.id == data.await.get::<BotId>().unwrap() {
        return;
    }

    if new_m.author.id.to_user(&ctx).await.unwrap().bot {
        return;
    }

    if check_log_type(LogType::MessageEdited, guildid).await.is_err() {
        return;
    }

    let _ = log_channel_say(
        &ctx,
        guildid,
        &format!(
            "Message by {} updated in channel {} from:\n{}\nTo:\n{}",
            new_m.author,
            new_m.channel(&ctx.cache).await.unwrap(),
            old_m.content,
            new_m.content
        ),
    )
    .await;
}
