use crate::prelude::*;
use serenity::{model::prelude::*, prelude::*};

pub async fn guild_ban_addition(ctx: Context, guildid: GuildId, user: User) {
    if check_log_type(LogType::UserBanned, guildid).await.is_err() {
        return;
    }

    let log_channel = match get_log_channel(guildid).await {
        Ok(c) => c,
        Err(_) => return,
    };

    let avatar = user.face().replace("size=1024", "size=128");
    let mut msg = String::from("**User Banned**\n");
    msg += &format!("ID: {}\n", user.id);
    msg += &format!("Tag: {}\n", user.tag());
    msg += &format!("Ping: {}\n", user.mention());

    let _ = log_channel
        .send_message(&ctx, |message| {
            message.content(msg);
            message.add_file(&avatar[..]);
            message
        })
        .await;
}
