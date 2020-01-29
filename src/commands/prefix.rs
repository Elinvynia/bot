use crate::{data::*, db::*};
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};

#[command]
#[only_in(guilds)]
#[owners_only]
#[num_args(1)]
fn prefix(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let conn = match get_db() {
        Ok(c) => c,
        Err(_) => return Ok(()),
    };

    let mut data = ctx.data.write();
    let guildid = msg.guild_id.unwrap();
    let pref = args.current().unwrap_or("!");

    let _ = conn.execute(
        "INSERT OR REPLACE INTO prefix (guild_id, prefix) values (?1, ?2)",
        &[&guildid.as_u64().to_string(), pref],
    );

    let prefixes = data.get_mut::<Prefix>().unwrap();
    prefixes.insert(guildid, pref.to_string());

    let _ = msg
        .channel_id
        .say(&ctx.http, format!("The prefix has been set to: `{}`", pref));

    Ok(())
}
