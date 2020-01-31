use crate::data::{BotError, LeaderboardEntry, Prefix};
use log::error;
use rusqlite::{Connection, NO_PARAMS};
use serenity::{model::prelude::*, prelude::*};
use std::{fs::File, path::Path};

pub fn create_db() {
    let db = Path::new("db.sqlite3");
    if !db.exists() {
        match File::create(&db) {
            Ok(_) => (),
            Err(e) => error!("Failed to create database file: {}", e),
        }
    }
    if let Ok(connection) = Connection::open(&db) {
        match connection.execute(
            "CREATE TABLE IF NOT EXISTS log (guild_id TEXT PRIMARY KEY, channel_id TEXT NOT NULL, log_type TEXT NOT NULL);",
            NO_PARAMS,
        ) {
            Ok(_) => (),
            Err(e) => {
                error!("{}", e);
            }
        }
        match connection.execute(
            "CREATE TABLE IF NOT EXISTS prefix (guild_id TEXT PRIMARY KEY, prefix TEXT NOT NULL);",
            NO_PARAMS,
        ) {
            Ok(_) => (),
            Err(e) => {
                error!("{}", e);
            }
        }
        match connection.execute(
            "CREATE TABLE IF NOT EXISTS leaderboard (guild_id TEXT NOT NULL, user_id TEXT NOT NULL, channel_id TEXT NOT NULL, points INTEGER DEFAULT 0 NOT NULL, PRIMARY KEY (guild_id, user_id));",
            NO_PARAMS,
        ) {
            Ok(_) => (),
            Err(e) => {
                error!("{}", e);
            }
        }
    } else {
        error!(
            "Could not open connection to database ({})",
            &db.to_string_lossy()
        );
    }
}

pub fn get_db() -> Result<Connection, BotError> {
    let db = Path::new("db.sqlite3");
    match Connection::open(db) {
        Ok(c) => return Ok(c),
        Err(e) => return Err(BotError::DbError(e)),
    }
}

pub fn get_log_channel(guildid: &GuildId) -> Result<ChannelId, BotError> {
    let conn = match get_db() {
        Ok(c) => c,
        Err(e) => return Err(e),
    };

    let mut statement = conn.prepare("SELECT channel_id FROM log WHERE guild_id == ?1;")?;
    let mut rows = statement.query(&[&guildid.as_u64().to_string()])?;
    let result: String = rows.next()?.ok_or("Guild not found.".to_string())?.get(0)?;
    let cid: u64 = result.parse()?;

    Ok(ChannelId(cid))
}

pub fn get_log_type(guildid: &GuildId) -> Result<i64, BotError> {
    let conn = get_db()?;

    let mut statement = conn.prepare("SELECT log_type FROM log WHERE guild_id == ?1;")?;
    let mut rows = statement.query(&[&guildid.as_u64().to_string()])?;
    let result: String = rows.next()?.ok_or("Guild not found.".to_string())?.get(0)?;
    let log_type: i64 = result.parse()?;

    Ok(log_type)
}

pub fn get_prefix(guildid: &GuildId, ctx: &Context) -> Result<String, BotError> {
    let conn = get_db()?;
    let mut statement = conn.prepare("SELECT prefix FROM prefix WHERE guild_id == ?1;")?;
    let mut rows = statement.query(&[&guildid.as_u64().to_string()])?;
    let prefix: String = rows.next()?.ok_or("Guild not found.".to_string())?.get(0)?;
    {
        let mut data = ctx.data.write();
        let prefixes = data.get_mut::<Prefix>().unwrap();
        prefixes.insert(guildid.clone(), prefix.clone());
    }
    Ok(prefix)
}

pub fn get_user_score(guildid: &GuildId, userid: &UserId) -> Result<i64, BotError> {
    let conn = get_db()?;
    let mut statement =
        conn.prepare("SELECT points FROM leaderboard WHERE guild_id == ?1 AND user_id == ?2;")?;
    let mut rows =
        statement.query(&[&guildid.as_u64().to_string(), &userid.as_u64().to_string()])?;
    Ok(rows.next()?.ok_or("No record yet.".to_string())?.get(0)?)
}

pub fn get_user_scores(guildid: &GuildId) -> Result<Vec<LeaderboardEntry>, BotError> {
    let guild_id = guildid.as_u64().to_string();
    let conn = get_db()?;
    let mut statement = conn.prepare("SELECT user_id, points FROM leaderboard WHERE guild_id == ?1 ORDER BY points DESC LIMIT 10;")?;
    let result_iter = statement.query_map(&[&guild_id], |row| {
        Ok(LeaderboardEntry {
            user_id: row.get(0)?,
            channel_id: row.get(1)?,
            points: row.get(2)?,
        })
    })?;

    let mut result = Vec::new();
    for x in result_iter {
        result.push(x?);
    }

    Ok(result)
}
