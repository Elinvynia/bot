use crate::data::LogType;
use crate::db::{get_log_channel, get_log_type};
use log::error;
use serenity::{model::prelude::*, prelude::*};
use std::sync::Arc;

pub fn channel_create(ctx: Context, channel: Arc<RwLock<GuildChannel>>) {
    let c = channel.read();
    let guildid = c.guild_id;

    let log_channel = match get_log_channel(&guildid) {
        Ok(l) => l,
        Err(_) => {
            return;
        }
    };

    let log_type = match get_log_type(&guildid) {
        Ok(l) => l,
        Err(_) => {
            return;
        }
    };

    if log_type & LogType::ChannelCreated as i64 != LogType::ChannelCreated as i64 {
        return;
    }

    if let Err(e) = log_channel.say(&ctx.http, format!("Channel created: {}", c.name)) {
        error!("{:?}", e);
    }
}
