use crate::data::cache::Pool;
use crate::data::error::BotError;
use ::log::error;
use config::Config;
use serenity::prelude::*;

use sqlx::pool::PoolConnection;
use sqlx::prelude::*;

#[cfg(feature = "sqlite")]
use sqlx::sqlite::{SqliteConnection, SqlitePool};
#[cfg(feature = "sqlite")]
use std::{fs::File, path::Path};

#[cfg(feature = "postgres")]
use sqlx::postgres::{PgConnection, PgPool};
#[cfg(feature = "postgres")]
use std::env;

pub mod leaderboard;
pub mod log;
pub mod prefix;

pub async fn create_db(settings: &Config) {
    match connect(settings).await {
        Ok(mut conn) => {
            match sqlx::query("CREATE TABLE IF NOT EXISTS log (guild_id TEXT PRIMARY KEY, channel_id TEXT NOT NULL, log_type TEXT NOT NULL);")
            .execute(&mut conn).await
            {
                Ok(_) => (),
                Err(e) => {
                    error!("Failed to crate table `log`: {}", e);
                }
            };
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
            };
            match sqlx::query("CREATE TABLE IF NOT EXISTS leaderboard (guild_id TEXT NOT NULL, user_id TEXT NOT NULL, channel_id TEXT NOT NULL, points INTEGER DEFAULT 0 NOT NULL, PRIMARY KEY (guild_id, user_id, channel_id));")
            .execute(&mut conn).await
            {
                Ok(_) => (),
                Err(e) => {
                    error!("Failed to create table `leaderboard`: {}", e);
                }
            };
            match sqlx::query("CREATE TABLE IF NOT EXISTS money (guild_id TEXT NOT NULL, user_id TEXT NOT NULL, money INTEGER DEFAULT 0 NOT NULL, PRIMARY KEY (guild_id, user_id));")
            .execute(&mut conn).await
            {
                Ok(_) => (),
                Err(e) => {
                    error!("Failed to create table `money`: {}", e);
                }
            };
        }
        Err(e) => {
            error!("Could not open connection to database ({})", e);
        }
    }
}

#[cfg(feature = "sqlite")]
pub async fn connect(_: &Config) -> Result<SqliteConnection, BotError> {
    let db = Path::new("db.sqlite3");
    if !db.exists() {
        match File::create(&db) {
            Ok(_) => (),
            Err(e) => error!("Failed to create database file: {}", e),
        }
    };
    Ok(SqliteConnection::connect("sqlite://db.sqlite3").await?)
}

#[cfg(feature = "postgres")]
pub async fn connect(settings: &Config) -> Result<PgConnection, BotError> {
    let user = if let Ok(x) = std::env::var("DB_USERNAME") {
        x
    } else {
        settings
            .get_str("username")
            .expect("No username in db.toml")
    };

    let password = if let Ok(x) = std::env::var("DB_PASSWORD") {
        x
    } else {
        settings
            .get_str("password")
            .expect("No password in db.toml")
    };

    let host = if let Ok(x) = std::env::var("DB_HOST") {
        x
    } else {
        settings.get_str("host").expect("No host in db.toml")
    };

    let database = if let Ok(x) = std::env::var("DB_DATABASE") {
        x
    } else {
        settings
            .get_str("database")
            .expect("No database in db.toml")
    };

    let url = format!("postgresql://{}:{}@{}/{}", user, password, host, database);

    PgConnection::connect(&url)
        .await
        .map_err(|e| BotError::DbError(e))
}

#[cfg(feature = "sqlite")]
pub async fn get_db(ctx: &Context) -> Result<PoolConnection<SqliteConnection>, BotError> {
    let data = ctx.data.read().await;
    let pool = data.get::<Pool>().unwrap();
    match pool.acquire().await {
        Ok(c) => Ok(c),
        Err(e) => Err(BotError::DbError(e)),
    }
}

#[cfg(feature = "postgres")]
pub async fn get_db(ctx: &Context) -> Result<PoolConnection<PgConnection>, BotError> {
    let data = ctx.data.read().await;
    let pool = data.get::<Pool>().unwrap();
    match pool.acquire().await {
        Ok(c) => Ok(c),
        Err(e) => Err(BotError::DbError(e)),
    }
}

#[cfg(feature = "sqlite")]
pub async fn create_pool(_: &Config) -> SqlitePool {
    sqlx::SqlitePool::new("sqlite://db.sqlite3")
        .await
        .expect("Failed to create DB pool")
}

#[cfg(feature = "postgres")]
pub async fn create_pool(settings: &Config) -> PgPool {
    let user = if let Ok(x) = std::env::var("DB_USERNAME") {
        x
    } else {
        settings
            .get_str("username")
            .expect("No username in db.toml")
    };

    let password = if let Ok(x) = env::var("DB_PASSWORD") {
        x
    } else {
        settings
            .get_str("password")
            .expect("No password in db.toml")
    };

    let host = if let Ok(x) = env::var("DB_HOST") {
        x
    } else {
        settings.get_str("host").expect("No host in db.toml")
    };

    let database = if let Ok(x) = env::var("DB_DATABASE") {
        x
    } else {
        settings
            .get_str("database")
            .expect("No database in db.toml")
    };

    let url = format!("postgresql://{}:{}@{}/{}", user, password, host, database);

    sqlx::PgPool::new(&url)
        .await
        .expect("Failed to create DB pool")
}
