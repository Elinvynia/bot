use crate::prelude::*;
use serenity::{model::prelude::*, prelude::*};

pub async fn category_delete(ctx: Context, category: &ChannelCategory) {
    let guildid = category.guild_id;

    if check_log_type(LogType::CategoryDeleted, guildid).await.is_err() {
        return;
    }

    let _ = log_channel_say(&ctx, guildid, &format!("Category deleted: {}", category.name)).await;
}
