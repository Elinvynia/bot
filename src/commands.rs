use serenity::framework::standard::macros::group;

pub mod ping;
use ping::*;

pub mod prefix;
use prefix::*;

#[group]
#[commands(ping, prefix)]
struct General;

pub mod log;
use self::log::*;

#[group]
#[commands(log)]
struct Config;

pub mod ban;
use ban::*;

pub mod kick;
use kick::*;

#[group]
#[commands(ban, kick)]
struct Admin;
