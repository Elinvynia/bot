use serenity::framework::standard::macros::group;

pub mod ping;
use ping::*;

#[group]
#[commands(ping)]
struct General;

pub mod log;
use log::*;

#[group]
#[commands(log)]
struct Config;

pub mod ban;
use ban::*;

#[group]
#[commands(ban)]
struct Admin;
