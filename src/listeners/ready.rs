use crate::db::reactionroles::start_reactions;
use log::info;
use serenity::{model::prelude::*, prelude::*};

pub async fn ready(ctx: Context, ready: Ready) {
    info!("Connected as {}", ready.user.name);

    start_reactions(&ctx).await.expect("Failed to start reaction roles");
}
