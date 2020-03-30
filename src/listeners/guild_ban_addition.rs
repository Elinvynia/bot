use crate::data::db::LogType;
use crate::db::log::{get_log_channel, get_log_type};
use log::error;
use serenity::{model::prelude::*, prelude::*};

pub async fn guild_ban_addition(ctx: Context, guildid: GuildId, user: User) {
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

    if log_type & LogType::UserBanned as i64 != LogType::UserBanned as i64 {
        return;
    }

    let avatar = user.face().replace("size=1024", "size=128");

    if let Err(e) = log_channel
        .send_message(&ctx.http, |message| {
            message.content(format!(
                "User banned:\nTag: {}\nID: {}",
                user.tag(),
                user.id
            ));
            message.add_file(&avatar[..]);
            message
        })
        .await
    {
        error!("{:?}", e);
    }
}
