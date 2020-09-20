use crate::data::error::BotError;
use std::{convert::TryFrom, ops::*};

#[derive(Debug, sqlx::FromRow)]
pub struct LeaderboardEntry {
    pub user_id: String,
    pub points: i64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
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
    UserUpdated = 1 << 10,
    VoiceUpdate = 1 << 11,
    PresenceUpdate = 1 << 12,
    All = (1 << 13) - 1,
}

impl std::fmt::Display for LogType {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let msg = match self {
            LogType::MessageDeleted => "Deleted",
            LogType::MessageEdited => "Edited",
            LogType::UserJoined => "User join",
            LogType::UserLeft => "User leave",
            LogType::UserBanned => "User ban",
            LogType::ChannelCreated => "Channel creations",
            LogType::ChannelDeleted => "Channel deletion",
            LogType::CategoryCreated => "Category creation",
            LogType::CategoryDeleted => "Category deletion",
            LogType::UserUpdated => "User update",
            LogType::VoiceUpdate => "Voice update",
            LogType::PresenceUpdate => "Prese update",
            LogType::All => "All",
        };
        write!(fmt, "{}", msg)
    }
}

impl TryFrom<String> for LogType {
    type Error = BotError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "delete" => Ok(LogType::MessageDeleted),
            "edit" => Ok(LogType::MessageEdited),
            "join" => Ok(LogType::UserJoined),
            "ban" => Ok(LogType::UserBanned),
            "chancreate" => Ok(LogType::ChannelCreated),
            "chandelete" => Ok(LogType::ChannelDeleted),
            "catcreate" => Ok(LogType::CategoryCreated),
            "catdelete" => Ok(LogType::CategoryDeleted),
            "update" => Ok(LogType::UserUpdated),
            "voiceupdate" => Ok(LogType::VoiceUpdate),
            "presenceupdate" => Ok(LogType::PresenceUpdate),
            "all" => Ok(LogType::All),
            _ => Err(BotError::LogTypeNotFound),
        }
    }
}


#[sqlx(transparent)]
#[derive(Eq, PartialEq, Debug, Copy, Clone, Default, sqlx::Type, Ord, PartialOrd)]
pub struct Money(pub u64);

impl Deref for Money {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for Money {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "{}$", self.0)
    }
}

impl std::str::FromStr for Money {
    type Err = BotError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num: u64 = s.parse()?;
        Ok(Money(num))
    }
}

impl Add for Money {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl Sub for Money {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0)
    }
}

impl Mul for Money {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self(self.0 * other.0)
    }
}

impl From<i64> for Money {
    fn from(num: i64) -> Self {
        Self(num as u64)
    }
}

impl From<u64> for Money {
    fn from(num: u64) -> Self {
        Self(num)
    }
}

impl From<u32> for Money {
    fn from(num: u32) -> Self {
        Self(num as u64)
    }
}

impl From<u16> for Money {
    fn from(num: u16) -> Self {
        Self(num as u64)
    }
}

impl From<u8> for Money {
    fn from(num: u8) -> Self {
        Self(num as u64)
    }
}

