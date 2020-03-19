use crate::{data::cache::GuildPrefixes, db::get_db};
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};

#[command]
#[only_in(guilds)]
#[owners_only]
#[num_args(1)]
async fn prefix(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let conn = get_db()?;
    let guildid = msg.guild_id.unwrap();
    let pref = args.current().await.unwrap_or("!");

    let _ = conn.execute(
        "INSERT OR REPLACE INTO prefix (guild_id, prefix) values (?1, ?2)",
        &[&guildid.as_u64().to_string(), pref],
    );

    {
        let mut data = ctx.data.write().await;
        let prefixes = data.get_mut::<GuildPrefixes>().unwrap();
        prefixes.insert(guildid, pref.to_string());
    }

    msg
        .channel_id
        .say(&ctx.http, format!("The prefix has been set to: `{}`", pref)).await?;

    Ok(())
}
