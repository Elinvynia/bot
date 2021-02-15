use crate::prelude::*;
use serenity::{
    client::bridge::gateway::ShardId,
    framework::standard::{macros::command, CommandResult},
    model::prelude::*,
    prelude::*,
};

#[command]
#[description = "Retrieves the current shard latency."]
#[usage = "ping"]
#[example = "ping"]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let shard_manager = data.get::<ShardManagerContainer>().ok_or(anyhow!("Shard manager container not found."))?;

    let manager = shard_manager.lock().await;
    let runners = manager.runners.lock().await;
    let runner = runners.get(&ShardId(ctx.shard_id)).ok_or(anyhow!("Shard not found."))?;

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
