use log::info;
use serenity::{model::prelude::*, prelude::*};

pub fn ready(_: Context, ready: Ready) {
    info!("Connected as {}", ready.user.name);
}
