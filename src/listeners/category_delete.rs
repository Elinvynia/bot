use serenity::{model::prelude::*, prelude::*};

use crate::data::LogType;
use crate::db::{get_log_channel, get_log_type};
use log::error;
use std::sync::Arc;

pub fn category_delete(ctx: Context, category: Arc<RwLock<ChannelCategory>>) {
    let c = category.read();
    let guildid =
        c.id.to_channel(&ctx)
            .unwrap()
            .guild()
            .unwrap()
            .read()
            .guild_id;

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

    if log_type & LogType::CategoryDeleted as i64 != LogType::CategoryDeleted as i64 {
        return;
    }

    if let Err(e) = log_channel.say(&ctx.http, format!("Category deleted: {}", c.name)) {
        error!("{:?}", e);
    }
}