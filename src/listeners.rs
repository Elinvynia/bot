use serenity::{model::prelude::*, prelude::*};
use std::sync::Arc;

pub mod category_create;
pub mod category_delete;
pub mod channel_create;
pub mod channel_delete;
pub mod guild_ban_addition;
pub mod guild_member_addition;
pub mod guild_member_removal;
pub mod message;
pub mod message_delete;
pub mod message_update;
pub mod ready;

pub struct Handler;

impl EventHandler for Handler {
    fn category_create(&self, ctx: Context, category: Arc<RwLock<ChannelCategory>>) {
        category_create::category_create(ctx, category)
    }

    fn category_delete(&self, ctx: Context, category: Arc<RwLock<ChannelCategory>>) {
        category_delete::category_delete(ctx, category)
    }

    fn channel_create(&self, ctx: Context, channel: Arc<RwLock<GuildChannel>>) {
        channel_create::channel_create(ctx, channel)
    }

    fn channel_delete(&self, ctx: Context, channel: Arc<RwLock<GuildChannel>>) {
        channel_delete::channel_delete(ctx, channel)
    }

    fn guild_ban_addition(&self, ctx: Context, guildid: GuildId, user: User) {
        guild_ban_addition::guild_ban_addition(ctx, guildid, user)
    }

    fn guild_member_addition(&self, ctx: Context, guildid: GuildId, new_member: Member) {
        guild_member_addition::guild_member_addition(ctx, guildid, new_member)
    }

    fn guild_member_removal(&self, ctx: Context, gid: GuildId, user: User, member: Option<Member>) {
        guild_member_removal::guild_member_removal(ctx, gid, user, member)
    }

    fn message(&self, ctx: Context, new_message: Message) {
        message::message(ctx, new_message)
    }

    fn message_delete(&self, ctx: Context, channel: ChannelId, deleted_message_id: MessageId) {
        message_delete::message_delete(ctx, channel, deleted_message_id)
    }

    fn message_update(
        &self,
        ctx: Context,
        old: Option<Message>,
        new: Option<Message>,
        event: MessageUpdateEvent,
    ) {
        message_update::message_update(ctx, old, new, event)
    }

    fn ready(&self, ctx: Context, ready: Ready) {
        ready::ready(ctx, ready)
    }
}
