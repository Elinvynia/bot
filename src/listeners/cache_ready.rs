use crate::prelude::*;
use serenity::{model::prelude::*, prelude::*};
use std::collections::HashMap;

pub async fn cache_ready(ctx: Context, guilds: Vec<GuildId>) {
    info!("Cache ready: {:?}", guilds);

    let mut presences: HashMap<UserId, Presence> = HashMap::new();

    for g in guilds {
        let guild = match g.to_guild_cached(&ctx).await {
            Some(guild) => guild,
            None => return,
        };

        presences.extend(guild.presences);
    }

    info!("Presences loaded: {:?}", &presences.len());

    {
        let mut data = ctx.data.write().await;
        data.insert::<Presences>(presences);
    }
}
