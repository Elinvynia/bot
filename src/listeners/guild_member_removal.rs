use crate::data::db::LogType;
use crate::db::log::{get_log_channel, get_log_type};
use serenity::{model::prelude::*, prelude::*};

pub async fn guild_member_removal(
    ctx: Context,
    guildid: GuildId,
    user: User,
    _member: Option<Member>,
) {
    let log_channel = match get_log_channel(&guildid) {
        Ok(l) => l,
        Err(_) => {
            return;
        }
    };

    let log_type = match get_log_type(&guildid) {
        Ok(l) => l,
        Err(_) => {
            return;
        }
    };

    if log_type & LogType::UserLeft as i64 != LogType::UserLeft as i64 {
        return;
    }

    let avatar = user.face().replace("size=1024", "size=128");

    let _ = log_channel.send_message(&ctx.http, |message| {
        message.content(format!("User left:\nTag: {}\nID: {}", user.tag(), user.id));
        message.add_file(&avatar[..]);
        message
    });
}
