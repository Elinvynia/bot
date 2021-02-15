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
    let conn = connect()?;
    let gid = msg.guild_id.ok_or(anyhow!("Guild ID not found."))?;

    sql_block!({
        let mut s = conn.prepare("REMOVE FROM joinrole WHERE guild_id = ?1;")?;
        s.execute(params![gid.to_string()])?;
    })?;

    msg.channel_id.say(&ctx, "Join role removed!").await?;

    Ok(())
}
