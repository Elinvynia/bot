use crate::data::db::LogType;
use crate::db::log::{check_log_type, get_log_channel};
use serenity::{model::prelude::*, prelude::*};

pub async fn guild_member_update(ctx: Context, old_if_available: Option<Member>, new: Member) {
    if old_if_available.is_none() {
        return;
    };
    let old = old_if_available.unwrap();
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

    let url = new.user.avatar_url().unwrap();

    let _ = log_channel.send_message(&ctx, |message| {
        let mut content = String::new();
        content.push_str("User Updated\n");
        content.push_str(&format!("Username: {}\n", new.user.name));
        content.push_str(&format!("ID: {}\n", new.user.id));
        if username_changed {
            content.push_str(&format!("Username changed: {} to {}\n", old.user.name, new.user.name));
        };
        if nickname_changed {
            content.push_str(&format!("Nickname changed: {} to {}\n", old.nick.unwrap_or("None".into()), new.nick.unwrap_or("None".into())));
        };
        if avatar_changed {
            content.push_str("Avatar changed.\n");
            message.add_file(url.as_str());
        };
        message.content(content);
        message
    }).await;
}
