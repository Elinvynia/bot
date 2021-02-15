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
#[description("Chooses one option from a list.")]
#[usage("choose <options>")]
#[example("choose \"have dinner\" \"go to sleep\"")]
async fn choose(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let parsed_args: Result<Vec<String>, _> = args.quoted().iter().quoted().collect();
    let options = error_return_ok!(parsed_args);

    let chosen = options.choose(&mut rand::thread_rng()).ok_or(anyhow!("Failed to choose an option"))?;

    msg.channel_id.say(&ctx, chosen).await?;

    Ok(())
}
