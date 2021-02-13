use crate::prelude::*;
use serenity::{model::prelude::*, prelude::*};

pub async fn presence_update(ctx: Context, new_data: PresenceUpdateEvent) {
    let gid = match new_data.guild_id {
        Some(id) => id,
        None => return,
    };

    if check_log_type(LogType::PresenceUpdate, gid).await.is_err() {
        return;
    }

    let uid = new_data.presence.user_id;
    let new_presence = new_data.presence;

    let user = match uid.to_user_cached(&ctx).await {
        Some(u) => u,
        None => return,
    };

    // Fetch the old presence for comparison.
    let old_presence;
    {
        let data = ctx.data.read().await;
        let presences = match data.get::<Presences>() {
            Some(p) => p,
            None => return,
        };
        old_presence = match presences.get(&uid) {
            Some(p) => p.clone(),
            None => return,
        };
    }

    // Update the cache with the new presence.
    {
        let mut data = ctx.data.write().await;
        let presences = match data.get_mut::<Presences>() {
            Some(p) => p,
            None => return,
        };
        presences.insert(uid, new_presence.clone());
    }

    let mut message = String::from("Presence Update\n");
    message += &format!("ID: {}\n", user.id);
    message += &format!("Username: {}\n", user.tag());

    let mut change = false;
    if old_presence.status != new_presence.status {
        message += &format!("Status is now {:?} ", new_presence.status);
        match new_presence.client_status {
            Some(ClientStatus { desktop: Some(_), .. }) => message += "on Desktop",
            Some(ClientStatus { mobile: Some(_), .. }) => message += "on Mobile",
            Some(ClientStatus { web: Some(_), .. }) => message += "on Web",
            _ => {}
        };
        message += "\n";
        change = true;
    };

    if !change {
        return;
    };

    let _ = log_channel_say(&ctx, gid, &message).await;
}
