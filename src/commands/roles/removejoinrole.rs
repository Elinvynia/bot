use crate::prelude::*;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};

#[command]
#[only_in(guilds)]
#[owners_only]
#[description("Removes a join role from the current guild.")]
#[usage("removejoinrole")]
#[example("removejoinrole")]
async fn removejoinrole(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let mut conn = connect().await?;
    let gid = msg.guild_id.ok_or_else(|| anyhow!("Guild ID not found."))?;

    let gid = gid.to_string();
    sqlx::query!("DELETE FROM joinrole WHERE guild_id = ?1;", gid)
        .execute(&mut conn)
        .await?;

    msg.channel_id.say(&ctx, "Join role removed!").await?;

    Ok(())
}
