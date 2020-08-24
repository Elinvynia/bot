use serenity::framework::standard::macros::group;

pub mod general;
use general::ping::*;

#[group]
#[commands(ping)]
struct General;

pub mod roles;
use roles::addjoinrole::*;
use roles::addreactrole::*;
use roles::removejoinrole::*;
use roles::removereactrole::*;

#[group]
#[commands(addreactrole, removereactrole, addjoinrole, removejoinrole)]
struct Roles;

pub mod settings;
use settings::log::*;
use settings::prefix::*;

#[group]
#[commands(log, prefix)]
struct Settings;

pub mod admin;
use admin::ban::*;
use admin::kick::*;

#[group]
#[commands(ban, kick)]
struct Admin;

pub mod fun;
use fun::avatar::*;
use fun::emoji::*;
use fun::guild::*;
use fun::leaderboard::*;
use fun::user::*;

#[group]
#[commands(avatar, user, leaderboard, guild, emoji)]
struct Fun;

pub mod help;
