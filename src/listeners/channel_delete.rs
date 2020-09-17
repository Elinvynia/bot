use crate::prelude::*;
use serenity::{model::prelude::*, prelude::*};

pub async fn channel_delete(ctx: Context, channel: &GuildChannel) {
    let guildid = channel.guild_id;

    if check_log_type(LogType::ChannelDeleted, guildid).await.is_err() {
        return;
    }

    let mut msg = String::from("**Channel Deleted**\n");
    msg += &format!("ID: {}\n", channel.id);
    msg += &format!("Name: {}\n", channel.name);
    msg += &format!("NSFW: {}\n", channel.nsfw);

    let _ = log_channel_say(&ctx, guildid, &msg).await;
}
