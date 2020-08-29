use crate::prelude::*;
use serenity::{model::prelude::*, prelude::*};

pub async fn category_create(ctx: Context, category: &ChannelCategory) {
    let guildid = category.guild_id;

    if check_log_type(LogType::CategoryCreated, guildid).await.is_err() {
        return;
    }

    let _ = log_channel_say(&ctx, guildid, &format!("Category created: {}", category.name)).await;
}
