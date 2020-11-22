use crate::prelude::*;
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

    if old_m.content == new_m.content {
        return;
    }

    let botowners;
    {
        let data = ctx.data.read().await;
        botowners = none_return!(data.get::<BotOwners>()).clone();
    }

    let mut message = String::from("**Message Updated**\n");
    message += &format!("ID: {}\n", new_m.author.id);
    message += &format!("Tag: {}\n", new_m.author.tag());
    if botowners.iter().find(|&&id| id == new_m.author.id).is_none() {
        message += &format!("Ping: {}\n", new_m.author.mention());
    };
    message += &format!("Channel: {}\n", channel);
    message += &format!("Old Message: \n{}\n", old_m.content);
    message += "---\n";
    message += &format!("New Message: \n{}\n", new_m.content);

    let _ = log_channel_say(&ctx, guildid, &message).await;
}
