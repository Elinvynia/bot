use crate::data::db::LogType;
use crate::db::log::{check_log_type, log_channel_say};
use serenity::{model::prelude::*, prelude::*};

pub async fn presence_update(ctx: Context, new_data: PresenceUpdateEvent) {
    let guildid = match new_data.guild_id {
        Some(g) => g,
        None => { return; }
    };

    if check_log_type(LogType::Presence, guildid)
        .await
        .is_err()
    {
        return;
    }

    let _ = log_channel_say(
        &ctx,
        guildid,
        &format!("Presence updated: {:?}", new_data.presence),
    )
    .await;
}
