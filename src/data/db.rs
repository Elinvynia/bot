#[derive(Debug, sqlx::FromRow)]
pub struct LeaderboardEntry {
    pub user_id: String,
    pub points: i64,
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
