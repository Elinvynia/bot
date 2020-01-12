use crate::functions::*;
use crate::data::*;

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

        if !check_log_channel(&ctx, &guildid) {
            return;
        }

        if get_log_type(&ctx, &guildid) & LogType::MessageDeleted as i64 != LogType::MessageDeleted as i64 {
            return;
        }

        let log_channel = get_log_channel(&ctx, &guildid);

        if let Some(x) = ctx.cache.read().message(&channel, &deleted_message_id) {
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
}
