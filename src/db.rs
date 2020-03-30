use crate::data::cache::Pool;
use crate::data::error::BotError;
use ::log::error;
use serenity::prelude::*;
use std::{fs::File, path::Path};

use sqlx::pool::PoolConnection;
use sqlx::prelude::*;
use sqlx::sqlite::SqliteConnection;

pub mod leaderboard;
pub mod log;
pub mod prefix;

pub async fn create_db() {
    let db = Path::new("db.sqlite3");
    if !db.exists() {
        match File::create(&db) {
            Ok(_) => (),
            Err(e) => error!("Failed to create database file: {}", e),
        }
    }
    if let Ok(mut conn) = SqliteConnection::connect("sqlite://db.sqlite3").await {
        match sqlx::query("CREATE TABLE IF NOT EXISTS log (guild_id TEXT PRIMARY KEY, channel_id TEXT NOT NULL, log_type TEXT NOT NULL);")
        .execute(&mut conn).await
        {
            Ok(_) => (),
            Err(e) => {
                error!("Failed to crate table `log`: {}", e);
            }
        }
        match sqlx::query(
            "CREATE TABLE IF NOT EXISTS prefix (guild_id TEXT PRIMARY KEY, prefix TEXT NOT NULL);",
        )
        .execute(&mut conn)
        .await
        {
            Ok(_) => (),
            Err(e) => {
                error!("Failed to create table `prefix`: {}", e);
            }
        }
        match sqlx::query("CREATE TABLE IF NOT EXISTS leaderboard (guild_id TEXT NOT NULL, user_id TEXT NOT NULL, channel_id TEXT NOT NULL, points INTEGER DEFAULT 0 NOT NULL, PRIMARY KEY (guild_id, user_id, channel_id));")
        .execute(&mut conn).await
        {
            Ok(_) => (),
            Err(e) => {
                error!("Failed to create table `leaderboard`: {}", e);
            }
        }
        match sqlx::query("CREATE TABLE IF NOT EXISTS money (guild_id TEXT NOT NULL, user_id TEXT NOT NULL, money INTEGER DEFAULT 0 NOT NULL, PRIMARY KEY (guild_id, user_id));")
        .execute(&mut conn).await
        {
            Ok(_) => (),
            Err(e) => {
                error!("Failed to create table `money`: {}", e);
            }
        }
    } else {
        error!(
            "Could not open connection to database ({})",
            &db.to_string_lossy()
        );
    }
}

pub async fn get_db(ctx: &Context) -> Result<PoolConnection<SqliteConnection>, BotError> {
    let data = ctx.data.read().await;
    let pool = data.get::<Pool>().unwrap();
    match pool.acquire().await {
        Ok(c) => Ok(c),
        Err(e) => Err(BotError::DbError(e)),
    }
}
