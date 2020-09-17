use crate::prelude::*;
use serenity::{model::prelude::*, prelude::*};

pub async fn guild_member_update(ctx: Context, old_if_available: Option<Member>, new: Member) {
    let old = match old_if_available {
        Some(m) => m,
        None => return,
    };
    let guildid = new.guild_id;

    if check_log_type(LogType::UserUpdated, guildid).await.is_err() {
        return;
    }

    let log_channel = match get_log_channel(guildid).await {
        Ok(c) => c,
        Err(_) => return,
    };

    let username_changed = old.user.name != new.user.name;
    let nickname_changed = old.nick != new.nick;
    let avatar_changed = old.user.avatar != new.user.avatar;

    if !username_changed && !nickname_changed && !avatar_changed {
        return;
    };

    let url = match new.user.avatar_url() {
        Some(u) => u,
        None => return,
    };

    let _ = log_channel
        .send_message(&ctx, |message| {
            let mut content = String::from("**User Updated**\n");
            content += &format!("Username: {}\n", new.user.name);
            content += &format!("ID: {}\n", new.user.id);
            if username_changed {
                content += &format!("Username changed: {} to {}\n", old.user.name, new.user.name);
            };
            if nickname_changed {
                content += &format!(
                    "Nickname changed: {} to {}\n",
                    old.nick.unwrap_or_else(|| "None".into()),
                    new.nick.unwrap_or_else(|| "None".into())
                );
            };
            if avatar_changed {
                content += "Avatar changed.\n";
                message.add_file(url.as_str());
            };
            message.content(content);
            message
        })
        .await;
}
