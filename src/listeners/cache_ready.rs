use log::info;
use serenity::{model::prelude::*, prelude::*};

pub async fn cache_ready(_ctx: Context, guilds: Vec<GuildId>) {
    info!("Cache ready: {:?}", guilds);
}
