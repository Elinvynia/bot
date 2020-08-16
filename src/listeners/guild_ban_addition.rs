use crate::data::db::LogType;
use crate::db::log::{check_log_type, get_log_channel};
use serenity::{model::prelude::*, prelude::*};

pub async fn guild_ban_addition(ctx: Context, guildid: GuildId, user: User) {
    if check_log_type(LogType::UserBanned, guildid).await.is_err() {
        return;
    }

    let log_channel = get_log_channel(guildid).await.unwrap();
    let avatar = user.face().replace("size=1024", "size=128");

    let _ = log_channel
        .send_message(&ctx, |message| {
            message.content(format!("User banned:\nTag: {}\nID: {}", user.tag(), user.id));
            message.add_file(&avatar[..]);
            message
        })
        .await;
}
