use serenity::framework::standard::macros::group;
pub mod help;
pub use help::*;

pub mod generic;
pub use generic::*;

pub mod admin;
pub use admin::*;
#[group]
#[commands(ban, kick)]
struct Admin;

pub mod games;
pub use games::*;
#[group]
#[commands(setmoney, betroll, money, flip, give)]
struct Gambling;

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

pub mod utility;
pub use utility::*;
#[group]
#[commands(avatar, user, leaderboard, guild, emoji, choose, ping, hug, answer, headpat, bonk)]
struct Utility;
