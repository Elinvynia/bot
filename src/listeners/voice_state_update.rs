use crate::prelude::*;
use serenity::{model::prelude::*, prelude::*};

pub async fn voice_state_update(ctx: Context, gid: Option<GuildId>, old: Option<VoiceState>, new: VoiceState) {
    let gid = match gid {
        Some(id) => id,
        None => return,
    };

    if check_log_type(LogType::VoiceUpdate, gid).await.is_err() {
        return;
    };

    let user = match new.user_id.to_user_cached(&ctx).await {
        Some(u) => u,
        None => return,
    };

    let mut message = String::new();
    message.push_str("Voice State Updated\n");
    message.push_str(&format!("Username: {}\n", user.tag()));
    message.push_str(&format!("ID: {}\n", user.id));

    let old = match old {
        Some(o) => o,
        None => {
            if let Some(vc) = new.channel_id {
                message.push_str(&format!(
                    "Joined voice channel: {}\n",
                    vc.to_channel(&ctx)
                        .await
                        .ok()
                        .map_or("Failed to get channel name.".to_string(), |c| c.to_string())
                ));
                let _ = log_channel_say(&ctx, gid, &message).await;
            };
            return;
        }
    };

    if old.deaf != new.deaf {
        if old.deaf {
            message.push_str("User is no longer deafened.\n");
        } else {
            message.push_str("User has been deafened.\n");
        }
    };

    if old.mute != new.mute {
        if old.mute {
            message.push_str("User is no longer muted.\n");
        } else {
            message.push_str("User has been muted.\n");
        }
    };

    if old.self_deaf != new.self_deaf {
        if old.self_deaf {
            message.push_str("User is no longer self-deafened.\n");
        } else {
            message.push_str("User has been self-deafened.\n");
        }
    };

    if old.self_mute != new.self_mute {
        if old.self_mute {
            message.push_str("User is no longer self-muted.\n");
        } else {
            message.push_str("User has been self-muted.\n");
        }
    };

    if old.suppress != new.suppress {
        if old.suppress {
            message.push_str("User is no longer suppressed.\n");
        } else {
            message.push_str("User has been suppressed.\n");
        }
    };

    if old.channel_id != new.channel_id {
        if new.channel_id.is_none() {
            let cid = match old.channel_id {
                Some(id) => id,
                None => return,
            };
            message.push_str(&format!(
                "User left voice channel: {}\n",
                cid.to_channel(&ctx)
                    .await
                    .ok()
                    .map_or("Failed to get channel name".to_string(), |c| c.to_string())
            ));
        };

        if let Some(oid) = old.channel_id {
            if let Some(nid) = new.channel_id {
                message.push_str(&format!(
                    "User moved from channel {} to channel {}\n",
                    oid.to_channel(&ctx)
                        .await
                        .ok()
                        .map_or("Failed to get channel name".to_string(), |c| c.to_string()),
                    nid.to_channel(&ctx)
                        .await
                        .ok()
                        .map_or("Failed to get channel name".to_string(), |c| c.to_string()),
                ));
            }
        }
    };

    let _ = log_channel_say(&ctx, gid, &message).await;
}
