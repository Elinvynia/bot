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

    let _ = inc_user_channel_score(guild_id, new_message.channel_id, new_message.author.id).await;
    let _ = inc_user_money(guild_id, new_message.author.id).await;
}
