use crate::prelude::*;
use rand::prelude::*;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};

#[command]
#[min_args(1)]
#[max_args(99)]
#[description("Answers either yes or no to a question.")]
#[usage("answer <question>")]
#[example("answer ")]
async fn answer(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let answer = ["Yes", "No"]
        .choose(&mut rand::thread_rng())
        .ok_or_else(|| anyhow!("Failed to choose an option."))?;

    msg.channel_id.say(&ctx, answer).await?;

    Ok(())
}
