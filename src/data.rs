use serenity::{client::bridge::gateway::ShardManager, model::prelude::*, prelude::*};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
    num,
    sync::Arc,
};

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

#[derive(Debug)]
pub enum BotError {
    DbError(rusqlite::Error),
    ParseError(String),
    CustomError(&'static str),
}

impl Display for BotError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self)
    }
}

impl Error for BotError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl From<rusqlite::Error> for BotError {
    fn from(err: rusqlite::Error) -> BotError {
        return BotError::DbError(err);
    }
}

impl From<&'static str> for BotError {
    fn from(err: &'static str) -> BotError {
        return BotError::CustomError(err);
    }
}

impl From<num::ParseIntError> for BotError {
    fn from(err: num::ParseIntError) -> BotError {
        return BotError::ParseError(err.to_string());
    }
}
