use crate::db::*;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};

#[command]
#[only_in(guilds)]
#[owners_only]
#[num_args(1)]
fn prefix(_ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let conn = match get_db() {
        Ok(c) => c,
        Err(_) => return Ok(()),
    };

    let pref = args.current().unwrap_or("!");

    let _ = conn.execute(
        "INSERT OR REPLACE INTO prefix (guild_id, prefix) values (?1, ?2)",
        &[&msg.guild_id.unwrap().as_u64().to_string(), pref],
    );

    Ok(())
}
