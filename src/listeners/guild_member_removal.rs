use crate::prelude::*;
use serenity::{model::prelude::*, prelude::*};

pub async fn guild_member_removal(ctx: Context, guildid: GuildId, user: User, _member: Option<Member>) {
    if check_log_type(LogType::UserLeft, guildid).await.is_err() {
        return;
    }

    let log_channel = match get_log_channel(guildid).await {
        Ok(c) => c,
        Err(_) => return,
    };

    let avatar = user.face().replace("size=1024", "size=128");
    let mut msg = String::from("**User Left**\n");
    msg += &format!("ID: {}\n", user.id);
    msg += &format!("Tag: {}\n", user.tag());
    msg += &format!("Ping: {}\n", user.mention());

    let _ = log_channel
        .send_message(&ctx.http, |message| {
            message.content(msg);
            message.add_file(&avatar[..]);
            message
        })
        .await;
}
