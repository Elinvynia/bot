use serenity::{client::bridge::gateway::ShardManager, model::prelude::*, prelude::*};
use std::{collections::HashMap, sync::Arc};

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

pub struct DefaultPrefix;
impl TypeMapKey for DefaultPrefix {
    type Value = String;
}

pub struct GuildPrefixes;
impl TypeMapKey for GuildPrefixes {
    type Value = HashMap<GuildId, String>;
}

pub struct Presences;
impl TypeMapKey for Presences {
    type Value = HashMap<UserId, Presence>;
}
