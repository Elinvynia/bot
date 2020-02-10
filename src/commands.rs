use serenity::framework::standard::macros::group;

pub mod general;
use general::ping::*;

#[group]
#[commands(ping)]
struct General;

pub mod config;
use config::log::*;
use config::prefix::*;

#[group]
#[commands(log, prefix)]
struct Config;

pub mod admin;
use admin::ban::*;
use admin::kick::*;

#[group]
#[commands(ban, kick)]
struct Admin;

pub mod fun;
use fun::avatar::*;
use fun::guild::*;
use fun::leaderboard::*;
use fun::user::*;

#[group]
#[commands(avatar, user, leaderboard, guild)]
struct Fun;

pub mod help;
