use crate::db::connect;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};

#[command]
#[only_in(guilds)]
#[owners_only]
#[num_args(2)]
#[description("Sets the amount of money for a user.")]
#[usage("setmoney <user> <amount>")]
#[example("setmoney Elinvynia 1000")]
async fn setmoney(_ctx: &Context, _msg: &Message, _args: Args) -> CommandResult {
    let _conn = connect().await?;
    Ok(())
}
