use crate::data::cache::ShardManagerContainer;
use log::error;
use serenity::{
    client::bridge::gateway::ShardId,
    framework::standard::{macros::command, CommandResult},
    model::prelude::*,
    prelude::*,
};

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let shard_manager = match data.get::<ShardManagerContainer>() {
        Some(v) => v,
        None => {
            error!("There was a problem getting the shard manager.");
            return Ok(());
        }
    };

    let manager = shard_manager.lock().await;
    let runners = manager.runners.lock().await;
    let runner = match runners.get(&ShardId(ctx.shard_id)) {
        Some(runner) => runner,
        None => {
            error!("No shard found.");
            return Ok(());
        }
    };

    match runner.latency {
        Some(x) => {
            msg.channel_id
                .say(&ctx, &format!("The shard latency is {}ms.", x.as_millis()))
                .await?;
        }
        None => {
            msg.channel_id
                .say(&ctx, "Please wait until the shard measures the latency.")
                .await?;
        }
    };

    Ok(())
}
