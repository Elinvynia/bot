use crate::prelude::*;
use rand::prelude::*;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};

#[command]
#[only_in(guilds)]
#[min_args(1)]
#[max_args(99)]
#[description("Chooses one option from a list.")]
#[usage("choose <options>")]
#[example("choose \"have dinner\" \"go to sleep\"")]
async fn choose(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let _gid = msg.guild_id.ok_or(BotError::NoneError)?;

    let parsed_args: Result<Vec<String>, _> = args.quoted().iter().quoted().collect();
    let options = match parsed_args {
        Ok(o) => o,
        Err(_) => return Ok(()),
    };

    let chosen = options.choose(&mut rand::thread_rng()).ok_or(BotError::NoneError)?;

    msg.channel_id.say(&ctx, chosen).await?;

    Ok(())
}
