use crate::prelude::*;
use serenity::{model::prelude::*, prelude::*};

pub async fn voice_state_update(ctx: Context, gid: Option<GuildId>, old: Option<VoiceState>, new: VoiceState) {
    let gid = match gid {
        Some(id) => id,
        None => return,
    };

    if check_log_type(LogType::VoiceUpdate, gid).await.is_err() {
        return;
    }

    let _old = match old {
        Some(o) => o,
        None => return,
    };

    let _user = match new.user_id.to_user_cached(&ctx).await {
        Some(u) => u,
        None => return,
    };

    let _ = log_channel_say(&ctx, gid, "Voice state updated");
}
