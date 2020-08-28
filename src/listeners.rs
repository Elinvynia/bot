use serenity::{async_trait, model::prelude::*, prelude::*};

pub mod cache_ready;
pub mod category_create;
pub mod category_delete;
pub mod channel_create;
pub mod channel_delete;
pub mod guild_ban_addition;
pub mod guild_member_addition;
pub mod guild_member_removal;
pub mod guild_member_update;
pub mod message;
pub mod message_delete;
pub mod message_update;
pub mod presence_update;
pub mod ready;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn cache_ready(&self, ctx: Context, guilds: Vec<GuildId>) {
        cache_ready::cache_ready(ctx, guilds).await
    }

    async fn category_create(&self, ctx: Context, category: &ChannelCategory) {
        category_create::category_create(ctx, category).await
    }

    async fn category_delete(&self, ctx: Context, category: &ChannelCategory) {
        category_delete::category_delete(ctx, category).await
    }

    async fn channel_create(&self, ctx: Context, channel: &GuildChannel) {
        channel_create::channel_create(ctx, channel).await
    }

    async fn channel_delete(&self, ctx: Context, channel: &GuildChannel) {
        channel_delete::channel_delete(ctx, channel).await
    }

    async fn guild_ban_addition(&self, ctx: Context, guildid: GuildId, user: User) {
        guild_ban_addition::guild_ban_addition(ctx, guildid, user).await
    }

    async fn guild_member_addition(&self, ctx: Context, guildid: GuildId, new_member: Member) {
        guild_member_addition::guild_member_addition(ctx, guildid, new_member).await
    }

    async fn guild_member_removal(&self, ctx: Context, gid: GuildId, user: User, member: Option<Member>) {
        guild_member_removal::guild_member_removal(ctx, gid, user, member).await
    }

    async fn guild_member_update(&self, ctx: Context, old_if_available: Option<Member>, new: Member) {
        guild_member_update::guild_member_update(ctx, old_if_available, new).await
    }

    async fn message(&self, _ctx: Context, new_message: Message) {
        message::message(new_message).await
    }

    async fn message_delete(&self, ctx: Context, channel: ChannelId, deleted_message_id: MessageId) {
        message_delete::message_delete(ctx, channel, deleted_message_id).await
    }

    async fn message_update(
        &self,
        ctx: Context,
        old: Option<Message>,
        new: Option<Message>,
        event: MessageUpdateEvent,
    ) {
        message_update::message_update(ctx, old, new, event).await
    }

    async fn presence_update(&self, ctx: Context, new_data: PresenceUpdateEvent) {
        presence_update::presence_update(ctx, new_data).await
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        ready::ready(ctx, ready).await
    }
}
