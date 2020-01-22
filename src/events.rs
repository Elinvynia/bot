use crate::util::*;
use crate::data::*;

use std::sync::Arc;

use serenity::{
    model::{gateway::Ready, prelude::*},
    prelude::*,
};

pub struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);
    }

    fn message_delete(&self, ctx: Context, channel: ChannelId, deleted_message_id: MessageId) {
        let guildid = channel
            .to_channel(&ctx)
            .unwrap()
            .guild()
            .unwrap()
            .read()
            .guild_id;

        let log_c = get_log_channel(&guildid);
        if log_c.is_err() {
            return;
        }
        let log_channel = log_c.unwrap();

        if get_log_type(&guildid).unwrap() & LogType::MessageDeleted as i64 != LogType::MessageDeleted as i64 {
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

    fn message_update(&self, ctx: Context, old: Option<Message>, new: Option<Message>, _event: MessageUpdateEvent) {
        if old.is_none() || new.is_none() {
            return;
        }

        let old_m = old.unwrap();
        let new_m = new.unwrap();
        let guildid = new_m.guild_id.unwrap();

        let log_c = get_log_channel(&guildid);
        if log_c.is_err() {
            return;
        }
        let log_channel = log_c.unwrap();

        let data = ctx.data.read();
        if new_m.author.id == *data.get::<BotId>().unwrap() {
            return;
        }

        if new_m.guild_id.is_none() {
            return;
        }

        if get_log_type(&guildid).unwrap() & LogType::MessageEdited as i64 != LogType::MessageEdited as i64 {
            return;
        }

        let _ = log_channel.say(
            &ctx.http,
            format!(
                "Message by {} updated in channel {} from:\n{}\nTo:\n{}",
                new_m.author,
                new_m.channel(&ctx.cache).unwrap(),
                old_m.content,
                new_m.content)
        );
    }


    fn guild_member_addition(&self, ctx: Context, guildid: GuildId, new_member: Member) {
        let log_c = get_log_channel(&guildid);
        if log_c.is_err() {
            return;
        }
        let log_channel = log_c.unwrap();

        if get_log_type(&guildid).unwrap() & LogType::UserJoined as i64 != LogType::UserJoined as i64 {
            return;
        }

        let user = new_member.user.read();
        let mut picture: Vec<u8> = vec![];
        let _ = log_channel.send_message(
            &ctx.http, |message| {
                    let avatar = user.face().replace("size=1024", "size=128");
                    let mut req = reqwest::blocking::get(&avatar).unwrap();
                    let _ = std::io::copy(&mut req, &mut picture);
                    message.content(format!(
                        "User joined:\nTag: {}\nID: {}",
                        user.tag(),
                        user.id
                    ));
                    message.add_file((picture.as_slice(), format!("{}{}", user.tag(), ".webp").as_str()));
                    message
            }
        );
    }

    fn guild_member_removal(&self, ctx: Context, guildid: GuildId, user: User, _member: Option<Member>) {
        let log_c = get_log_channel(&guildid);
        if log_c.is_err() {
            return;
        }
        let log_channel = log_c.unwrap();

        if get_log_type(&guildid).unwrap() & LogType::UserLeft as i64 != LogType::UserLeft as i64 {
            return;
        }

        let mut picture: Vec<u8> = vec![];
        let _ = log_channel.send_message(
            &ctx.http, |message| {
                    let avatar = user.face().replace("size=1024", "size=128");
                    let mut req = reqwest::blocking::get(&avatar).unwrap();
                    let _ = std::io::copy(&mut req, &mut picture);
                    message.content(format!(
                        "User joined:\nTag: {}\nID: {}",
                        user.tag(),
                        user.id
                    ));
                    message.add_file((picture.as_slice(), format!("{}{}", user.tag(), ".webp").as_str()));
                    message
            }
        );
    }

    fn guild_ban_addition(&self, ctx: Context, guildid: GuildId, user: User) {
        let log_c = get_log_channel(&guildid);
        if log_c.is_err() {
            return;
        }
        let log_channel = log_c.unwrap();

        if get_log_type(&guildid).unwrap() & LogType::UserBanned as i64 != LogType::UserBanned as i64 {
            return;
        }

        let _ = log_channel.send_message(
            &ctx.http, |m| {
                if let Some(avatar) = user.avatar_url() {
                    m.content(format!(
                        "User banned:\nTag: {}\nID: {}",
                        user.tag(),
                        user.id
                    ));
                    m.embed(|e| {
                        let url = format!("{}{}", avatar, "?size=128");
                        e.image(url)
                    });
                }
                else {
                    m.content(format!(
                        "User left:\nTag: {}\nID: {}\nDefault avatar.",
                        user.tag(),
                        user.id
                    ));
                }
                m
            }
        );
    }

    fn channel_create(&self, ctx: Context, channel: Arc<RwLock<GuildChannel>>) {
        let c = channel.read();
        let guildid = c.guild_id;

        let log_c = get_log_channel(&guildid);
        if log_c.is_err() {
            return;
        }
        let log_channel = log_c.unwrap();

        if get_log_type(&guildid).unwrap() & LogType::ChannelCreated as i64 != LogType::ChannelCreated as i64 {
            return;
        }

        let _ = log_channel.say(
            &ctx.http,
            format!(
                "Channel created: {}",
                c.name)
        );
    }

    fn channel_delete(&self, ctx: Context, channel: Arc<RwLock<GuildChannel>>) {
        let c = channel.read();
        let guildid = c.guild_id;

        let log_c = get_log_channel(&guildid);
        if log_c.is_err() {
            return;
        }
        let log_channel = log_c.unwrap();

        if get_log_type(&guildid).unwrap() & LogType::ChannelDeleted as i64 != LogType::ChannelDeleted as i64 {
            return;
        }

        let _ = log_channel.say(
            &ctx.http,
            format!(
                "Channel deleted: {}",
                c.name)
        );
    }

    fn category_create(&self, ctx: Context, category: Arc<RwLock<ChannelCategory>>) {
        let c = category.read();
        let guildid = c.id
            .to_channel(&ctx)
            .unwrap()
            .guild()
            .unwrap()
            .read()
            .guild_id;

        let log_c = get_log_channel(&guildid);
        if log_c.is_err() {
            return;
        }
        let log_channel = log_c.unwrap();

        if get_log_type(&guildid).unwrap() & LogType::CategoryCreated as i64 != LogType::CategoryDeleted as i64 {
            return;
        }

        let _ = log_channel.say(
            &ctx.http,
            format!(
                "Category created: {}",
                c.name)
        );
    }


    fn category_delete(&self, ctx: Context, category: Arc<RwLock<ChannelCategory>>) {
        let c = category.read();
        let guildid = c.id
            .to_channel(&ctx)
            .unwrap()
            .guild()
            .unwrap()
            .read()
            .guild_id;

        let log_c = get_log_channel(&guildid);
        if log_c.is_err() {
            return;
        }
        let log_channel = log_c.unwrap();

        if get_log_type(&guildid).unwrap() & LogType::CategoryDeleted as i64 != LogType::CategoryDeleted as i64 {
            return;
        }

        let _ = log_channel.say(
            &ctx.http,
            format!(
                "Category deleted: {}",
                c.name)
        );
    }
}
