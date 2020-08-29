use crate::prelude::*;
use serenity::{model::prelude::*, prelude::*};

pub async fn channel_create(ctx: Context, channel: &GuildChannel) {
    let guildid = channel.guild_id;

    if check_log_type(LogType::ChannelCreated, guildid).await.is_err() {
        return;
    }

    let _ = log_channel_say(&ctx, guildid, &format!("Channel created: {}", channel.name)).await;
}
