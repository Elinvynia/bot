use serenity::{client::bridge::gateway::ShardManager, model::prelude::*, prelude::*};
use std::sync::Arc;

pub struct BotOwners;

impl TypeMapKey for BotOwners {
    type Value = Vec<UserId>;
}

pub struct BotId;

impl TypeMapKey for BotId {
    type Value = UserId;
}

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

pub enum LogType {
    MessageDeleted = 1 << 1,
    MessageEdited = 1 << 2,
    UserJoined = 1 << 3,
    UserLeft = 1 << 4,
    UserBanned = 1 << 5,
    ChannelCreated = 1 << 6,
    ChannelDeleted = 1 << 7,
    CategoryCreated = 1 << 8,
    CategoryDeleted = 1 << 9,
    All = (1 << 9) - 1,
}
