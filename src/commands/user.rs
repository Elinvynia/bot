use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};

#[command]
#[min_args(0)]
#[max_args(1)]
fn user(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    Ok(())
}
