use crate::data::db::LogType;
use crate::db::log::{get_log_channel, get_log_type};
use serenity::{model::prelude::*, prelude::*};

pub async fn guild_member_addition(ctx: Context, guildid: GuildId, new_member: Member) {
    let log_channel = match get_log_channel(&ctx, guildid).await {
        Ok(l) => l,
        Err(_) => {
            return;
        }
    };

    let log_type = match get_log_type(&ctx, guildid).await {
        Ok(l) => l,
        Err(_) => {
            return;
        }
    };

    if log_type & LogType::UserJoined as i64 != LogType::UserJoined as i64 {
        return;
    }

    let user = new_member.user;
    let avatar = user.face().replace("size=1024", "size=128");

    let _ = log_channel.send_message(&ctx.http, |message| {
        message.content(format!(
            "User joined:\nTag: {}\nID: {}",
            user.tag(),
            user.id
        ));
        message.add_file(&avatar[..]);
        message
    });
}
