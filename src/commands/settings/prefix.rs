use crate::prelude::*;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};

#[command]
#[only_in(guilds)]
#[owners_only]
#[num_args(1)]
#[description = "Sets the prefix for the current server."]
#[usage = "prefix <value>"]
#[example = "prefix !"]
async fn prefix(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut conn = connect().await?;
    let guildid = msg.guild_id.ok_or_else(|| anyhow!("Guild ID not found."))?;
    let pref = args.current().unwrap_or("!");

    let gid = guildid.to_string();
    sqlx::query!(
        "INSERT OR REPLACE INTO prefix (guild_id, prefix) values (?1, ?2)",
        gid,
        pref
    )
    .execute(&mut conn)
    .await?;

    {
        let mut data = ctx.data.write().await;
        let prefixes = data
            .get_mut::<GuildPrefixes>()
            .ok_or_else(|| anyhow!("Guild prefix not found."))?;
        prefixes.insert(guildid, pref.to_string());
    }

    msg.channel_id
        .say(&ctx.http, format!("The prefix has been set to: `{}`", pref))
        .await?;

    Ok(())
}
