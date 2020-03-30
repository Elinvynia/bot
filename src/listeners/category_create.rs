use crate::data::db::LogType;
use crate::db::log::{get_log_channel, get_log_type};
use log::error;
use serenity::{model::prelude::*, prelude::*};
use std::sync::Arc;

pub async fn category_create(ctx: Context, category: Arc<RwLock<ChannelCategory>>) {
    let c = category.read().await;
    let guildid =
        c.id.to_channel(&ctx)
            .await
            .unwrap()
            .guild()
            .unwrap()
            .read()
            .await
            .guild_id;

    let log_channel = match get_log_channel(&ctx, guildid).await {
        Ok(l) => l,
        Err(_) => {
            return;
        }
    };

    let log_type = match get_log_type(&ctx, guildid).await {
        Ok(l) => l,
        Err(_) => {
            return;
        }
    };

    if log_type & LogType::CategoryCreated as i64 != LogType::CategoryDeleted as i64 {
        return;
    }

    if let Err(e) = log_channel
        .say(&ctx.http, format!("Category created: {}", c.name))
        .await
    {
        error!("{:?}", e);
    }
}
