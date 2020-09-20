use serenity::framework::standard::macros::group;
pub mod help;
pub use help::*;

pub mod admin;
pub use admin::*;
#[group]
#[commands(ban, kick, purge)]
struct Admin;


pub mod gambling;
pub use gambling::*;
#[group]
#[commands(setmoney, betroll, money, flip)]
struct Gambling;


pub mod games;
pub use games::*;
#[group]
struct Games;


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
#[commands(avatar, user, leaderboard, guild, emoji, choose, ping)]
struct Utility;


#[group]
struct Xp;
