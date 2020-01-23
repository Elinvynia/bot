use log::error;
use rusqlite::{Connection, NO_PARAMS};
use std::{error::Error, fs::File, path::Path};

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
            "CREATE TABLE IF NOT EXISTS leaderboard (guild_id TEXT NOT NULL, user_id TEXT NOT NULL, points INTEGER DEFAULT 0 NOT NULL, PRIMARY KEY (guild_id, user_id));",
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

pub fn get_db() -> Result<Connection, Box<dyn Error>> {
    let db = Path::new("db.sqlite3");
    Ok(Connection::open(db)?)
}
