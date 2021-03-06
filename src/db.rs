use crate::prelude::*;
use std::{fs::File, path::Path};

pub mod leaderboard;
pub mod log;
pub mod money;
pub mod prefix;
pub mod reactionroles;

pub async fn setup_db() -> Result<()> {
    let mut conn = connect().await?;
    sqlx::query!(
        "CREATE TABLE IF NOT EXISTS log (guild_id TEXT PRIMARY KEY, channel_id TEXT NOT NULL, log_type TEXT NOT NULL);"
    )
    .execute(&mut conn)
    .await?;
    sqlx::query!("CREATE TABLE IF NOT EXISTS prefix (guild_id TEXT PRIMARY KEY, prefix TEXT NOT NULL);")
        .execute(&mut conn)
        .await?;
    sqlx::query!("CREATE TABLE IF NOT EXISTS leaderboard (guild_id TEXT NOT NULL, user_id TEXT NOT NULL, channel_id TEXT NOT NULL, points INTEGER DEFAULT 0 NOT NULL, PRIMARY KEY (guild_id, user_id, channel_id));")
    .execute(&mut conn).await?;
    sqlx::query!("CREATE TABLE IF NOT EXISTS money (guild_id TEXT NOT NULL, user_id TEXT NOT NULL, money INTEGER DEFAULT 0 NOT NULL, PRIMARY KEY (guild_id, user_id));")
    .execute(&mut conn).await?;
    sqlx::query!(
        "CREATE TABLE IF NOT EXISTS reactionroles (guild_id TEXT NOT NULL, message_id TEXT NOT NULL, role_id TEXT NOT NULL, reaction_id TEXT NOT NULL, PRIMARY KEY (guild_id, message_id, role_id, reaction_id));")
    .execute(&mut conn).await?;
    sqlx::query!(
        "CREATE TABLE IF NOT EXISTS joinrole (guild_id TEXT NOT NULL, role_id TEXT NOT NULL, PRIMARY KEY (guild_id, role_id));")
    .execute(&mut conn).await?;
    Ok(())
}

pub async fn connect() -> Result<SqliteConnection> {
    let db = Path::new("db.sqlite3");
    if !db.exists() {
        File::create(&db)?;
    };
    Ok(SqliteConnection::connect("sqlite://db.sqlite3").await?)
}
