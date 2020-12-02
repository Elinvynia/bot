use crate::prelude::*;
use serenity::model::prelude::*;

pub async fn message(new_message: Message) {
    let guild_id = match new_message.guild_id {
        Some(g) => g,
        None => return,
    };

    if new_message.author.bot {
        return;
    }

    let prefix = match get_prefix(guild_id).await {
        Ok(p) => p,
        Err(_) => "!".into(),
    };

    if new_message.content.starts_with(&prefix) {
        return;
    };

    let _ = inc_user_channel_score(guild_id, new_message.channel_id, new_message.author.id).await;
    let _ = inc_user_money(guild_id, new_message.author.id).await;
}
