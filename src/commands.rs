use serenity::framework::standard::macros::group;
pub mod help;
pub use help::*;

pub mod general;
pub use general::*;

#[group]
#[commands(ping)]
struct General;

pub mod roles;
pub use roles::*;

#[group]
#[commands(addreactrole, removereactrole, addjoinrole, removejoinrole)]
struct Roles;

pub mod settings;
pub use settings::*;

#[group]
#[commands(log, prefix)]
struct Settings;

pub mod admin;
pub use admin::*;

#[group]
#[commands(ban, kick, purge)]
struct Admin;

pub mod utilities;
pub use utilities::*;

#[group]
#[commands(avatar, user, leaderboard, guild, emoji, choose)]
struct Utilities;

pub mod fun;
pub use fun::*;

#[group]
#[commands(setmoney)]
struct Fun;
