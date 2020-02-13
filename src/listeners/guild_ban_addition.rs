use crate::data::db::LogType;
use crate::db::{get_log_channel, get_log_type};
use log::error;
use serenity::{model::prelude::*, prelude::*};

pub fn guild_ban_addition(ctx: Context, guildid: GuildId, user: User) {
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

    if log_type & LogType::UserBanned as i64 != LogType::UserBanned as i64 {
        return;
    }

    let mut picture: Vec<u8> = vec![];
    if let Err(e) = log_channel.send_message(&ctx.http, |message| {
        let avatar = user.face().replace("size=1024", "size=128");
        let mut req = reqwest::blocking::get(&avatar).unwrap();
        let _ = std::io::copy(&mut req, &mut picture);
        message.content(format!(
            "User banned:\nTag: {}\nID: {}",
            user.tag(),
            user.id
        ));
        message.add_file((
            picture.as_slice(),
            format!("{}{}", user.id, ".webp").as_str(),
        ));
        message
    }) {
        error!("{:?}", e);
    }
}
