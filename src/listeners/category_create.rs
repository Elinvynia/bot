use crate::prelude::*;
use serenity::{model::prelude::*, prelude::*};

pub async fn category_create(ctx: Context, category: &ChannelCategory) {
    let guildid = category.guild_id;

    if check_log_type(LogType::CategoryCreated, guildid).await.is_err() {
        return;
    }

    let mut msg = String::from("**Category Created**\n");
    msg += &format!("ID: {}\n", category.id);
    msg += &format!("Name: {}\n", category.name);
    msg += &format!("NSFW: {}\n", category.nsfw);

    let _ = log_channel_say(&ctx, guildid, &msg).await;
}
