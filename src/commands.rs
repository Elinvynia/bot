use serenity::framework::standard::macros::group;

pub mod ping;
use ping::*;

#[group]
#[commands(ping)]
struct General;

pub mod log;
use self::log::*;

pub mod prefix;
use prefix::*;

#[group]
#[commands(log, prefix)]
struct Config;

pub mod ban;
use ban::*;

pub mod kick;
use kick::*;

#[group]
#[commands(ban, kick)]
struct Admin;

pub mod avatar;
use avatar::*;

pub mod user;
use user::*;

pub mod leaderboard;
use leaderboard::*;

#[group]
#[commands(avatar, user, leaderboard)]
struct Fun;
