use crate::{data::*, db::*};
use log::{error, info};
use serenity::{
    model::{gateway::Ready, prelude::*},
    prelude::*,
};
use std::sync::Arc;

pub struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    fn message(&self, _ctx: Context, new_message: Message) {
        let guild_id = match new_message.guild_id {
            Some(g) => g,
            None => return,
        };

        let conn = match get_db() {
            Ok(c) => c,
            Err(e) => {
                error!("{:?}", e);
                return;
            }
        };

        if new_message.author.bot {
            return;
        }

        let user_id = new_message.author.id;

        match get_user_score(&guild_id, &user_id) {
            Ok(_) => {
                let _ = conn.execute("UPDATE leaderboard SET points = points + 1 WHERE guild_id == ?1 AND user_id == ?2;",
                                    &[&guild_id.as_u64().to_string(), &user_id.as_u64().to_string()]);
            }
            Err(BotError::CustomError(e)) if e == "No record yet." => {
                let _ = conn.execute(
                    "INSERT INTO leaderboard (guild_id, user_id, points) values (?1, ?2, 1);",
                    &[
                        &guild_id.as_u64().to_string(),
                        &user_id.as_u64().to_string(),
                    ],
                );
            }
            Err(e) => {
                error!("{:?}", e);
                return;
            }
        }
    }

    fn message_delete(&self, ctx: Context, channel: ChannelId, deleted_message_id: MessageId) {
        let guildid = channel
            .to_channel(&ctx)
            .unwrap()
            .guild()
            .unwrap()
            .read()
            .guild_id;

        let log_channel = match get_log_channel(&guildid) {
            Ok(l) => l,
            Err(e) => {
                error!("{:?}", e);
                return;
            }
        };

        let log_type = match get_log_type(&guildid) {
            Ok(l) => l,
            Err(e) => {
                error!("{:?}", e);
                return;
            }
        };

        if log_type & LogType::MessageDeleted as i64 != LogType::MessageDeleted as i64 {
            return;
        }

        if let Some(x) = ctx.cache.read().message(&channel, &deleted_message_id) {
            let data = ctx.data.read();
            if x.author.id == *data.get::<BotId>().unwrap() {
                return;
            }
            let _ = log_channel.say(
                &ctx.http,
                format!(
                    "Message by {} deleted in channel {}:\n{}",
                    x.author,
                    x.channel(&ctx.cache).unwrap(),
                    x.content
                ),
            );
        }
    }

    fn message_update(
        &self,
        ctx: Context,
        old: Option<Message>,
        new: Option<Message>,
        _event: MessageUpdateEvent,
    ) {
        if old.is_none() || new.is_none() {
            return;
        }

        let old_m = old.unwrap();
        let new_m = new.unwrap();
        let guildid = new_m.guild_id.unwrap();

        let log_channel = match get_log_channel(&guildid) {
            Ok(l) => l,
            Err(e) => {
                error!("{:?}", e);
                return;
            }
        };

        let data = ctx.data.read();
        if new_m.author.id == *data.get::<BotId>().unwrap() {
            return;
        }

        if new_m.guild_id.is_none() {
            return;
        }

        let log_type = match get_log_type(&guildid) {
            Ok(l) => l,
            Err(e) => {
                error!("{:?}", e);
                return;
            }
        };

        if log_type & LogType::MessageEdited as i64 != LogType::MessageEdited as i64 {
            return;
        }

        if let Err(e) = log_channel.say(
            &ctx.http,
            format!(
                "Message by {} updated in channel {} from:\n{}\nTo:\n{}",
                new_m.author,
                new_m.channel(&ctx.cache).unwrap(),
                old_m.content,
                new_m.content
            ),
        ) {
            error!("{:?}", e);
        }
    }

    fn guild_member_addition(&self, ctx: Context, guildid: GuildId, new_member: Member) {
        let log_channel = match get_log_channel(&guildid) {
            Ok(l) => l,
            Err(e) => {
                error!("{:?}", e);
                return;
            }
        };

        let log_type = match get_log_type(&guildid) {
            Ok(l) => l,
            Err(e) => {
                error!("{:?}", e);
                return;
            }
        };

        if log_type & LogType::UserJoined as i64 != LogType::UserJoined as i64 {
            return;
        }

        let user = new_member.user.read();
        let mut picture: Vec<u8> = vec![];
        let _ = log_channel.send_message(&ctx.http, |message| {
            let avatar = user.face().replace("size=1024", "size=128");
            let mut req = reqwest::blocking::get(&avatar).unwrap();
            let _ = std::io::copy(&mut req, &mut picture);
            message.content(format!(
                "User joined:\nTag: {}\nID: {}",
                user.tag(),
                user.id
            ));
            message.add_file((
                picture.as_slice(),
                format!("{}{}", user.id, ".webp").as_str(),
            ));
            message
        });
    }

    fn guild_member_removal(
        &self,
        ctx: Context,
        guildid: GuildId,
        user: User,
        _member: Option<Member>,
    ) {
        let log_channel = match get_log_channel(&guildid) {
            Ok(l) => l,
            Err(e) => {
                error!("{:?}", e);
                return;
            }
        };

        let log_type = match get_log_type(&guildid) {
            Ok(l) => l,
            Err(e) => {
                error!("{:?}", e);
                return;
            }
        };

        if log_type & LogType::UserLeft as i64 != LogType::UserLeft as i64 {
            return;
        }

        let mut picture: Vec<u8> = vec![];
        if let Err(e) = log_channel.send_message(&ctx.http, |message| {
            let avatar = user.face().replace("size=1024", "size=128");
            let mut req = reqwest::blocking::get(&avatar).unwrap();
            let _ = std::io::copy(&mut req, &mut picture);
            message.content(format!("User left:\nTag: {}\nID: {}", user.tag(), user.id));
            message.add_file((
                picture.as_slice(),
                format!("{}{}", user.id, ".webp").as_str(),
            ));
            message
        }) {
            error!("{:?}", e);
        }
    }

    fn guild_ban_addition(&self, ctx: Context, guildid: GuildId, user: User) {
        let log_channel = match get_log_channel(&guildid) {
            Ok(l) => l,
            Err(e) => {
                error!("{:?}", e);
                return;
            }
        };

        let log_type = match get_log_type(&guildid) {
            Ok(l) => l,
            Err(e) => {
                error!("{:?}", e);
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

    fn channel_create(&self, ctx: Context, channel: Arc<RwLock<GuildChannel>>) {
        let c = channel.read();
        let guildid = c.guild_id;

        let log_channel = match get_log_channel(&guildid) {
            Ok(l) => l,
            Err(e) => {
                error!("{:?}", e);
                return;
            }
        };

        let log_type = match get_log_type(&guildid) {
            Ok(l) => l,
            Err(e) => {
                error!("{:?}", e);
                return;
            }
        };

        if log_type & LogType::ChannelCreated as i64 != LogType::ChannelCreated as i64 {
            return;
        }

        if let Err(e) = log_channel.say(&ctx.http, format!("Channel created: {}", c.name)) {
            error!("{:?}", e);
        }
    }

    fn channel_delete(&self, ctx: Context, channel: Arc<RwLock<GuildChannel>>) {
        let c = channel.read();
        let guildid = c.guild_id;

        let log_channel = match get_log_channel(&guildid) {
            Ok(l) => l,
            Err(e) => {
                error!("{:?}", e);
                return;
            }
        };

        let log_type = match get_log_type(&guildid) {
            Ok(l) => l,
            Err(e) => {
                error!("{:?}", e);
                return;
            }
        };

        if log_type & LogType::ChannelDeleted as i64 != LogType::ChannelDeleted as i64 {
            return;
        }

        if let Err(e) = log_channel.say(&ctx.http, format!("Channel deleted: {}", c.name)) {
            error!("{:?}", e);
        }
    }

    fn category_create(&self, ctx: Context, category: Arc<RwLock<ChannelCategory>>) {
        let c = category.read();
        let guildid =
            c.id.to_channel(&ctx)
                .unwrap()
                .guild()
                .unwrap()
                .read()
                .guild_id;

        let log_channel = match get_log_channel(&guildid) {
            Ok(l) => l,
            Err(e) => {
                error!("{:?}", e);
                return;
            }
        };

        let log_type = match get_log_type(&guildid) {
            Ok(l) => l,
            Err(e) => {
                error!("{:?}", e);
                return;
            }
        };

        if log_type & LogType::CategoryCreated as i64 != LogType::CategoryDeleted as i64 {
            return;
        }

        if let Err(e) = log_channel.say(&ctx.http, format!("Category created: {}", c.name)) {
            error!("{:?}", e);
        }
    }

    fn category_delete(&self, ctx: Context, category: Arc<RwLock<ChannelCategory>>) {
        let c = category.read();
        let guildid =
            c.id.to_channel(&ctx)
                .unwrap()
                .guild()
                .unwrap()
                .read()
                .guild_id;

        let log_channel = match get_log_channel(&guildid) {
            Ok(l) => l,
            Err(e) => {
                error!("{:?}", e);
                return;
            }
        };

        let log_type = match get_log_type(&guildid) {
            Ok(l) => l,
            Err(e) => {
                error!("{:?}", e);
                return;
            }
        };

        if log_type & LogType::CategoryDeleted as i64 != LogType::CategoryDeleted as i64 {
            return;
        }

        if let Err(e) = log_channel.say(&ctx.http, format!("Category deleted: {}", c.name)) {
            error!("{:?}", e);
        }
    }
}
