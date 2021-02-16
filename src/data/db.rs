use crate::prelude::*;
use std::convert::TryFrom;

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
    type Error = anyhow::Error;
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
            _ => Err(anyhow!("Log type not found")),
        }
    }
}
