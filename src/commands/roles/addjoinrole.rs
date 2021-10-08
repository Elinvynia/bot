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
#[description("Automatically adds a role to newly joined users in the current guild.")]
#[usage("addjoinrole <role>")]
#[example("addjoinrole New")]
async fn addjoinrole(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut conn = connect().await?;
    let gid = msg.guild_id.ok_or_else(|| anyhow!("Guild ID not found."))?;

    let role = match parse_rol(&args.single::<String>()?, Some(&gid), Some(ctx)).await {
        Some(rid) => rid
            .to_role_cached(&ctx.cache)
            .await
            .ok_or_else(|| anyhow!("Role not found in cache"))?,
        None => return Ok(()),
    };

    let (gid, rid) = (gid.to_string(), role.id.to_string());
    sqlx::query!("INSERT INTO joinrole (guild_id, role_id) values (?1, ?2)", gid, rid)
        .execute(&mut conn)
        .await?;

    msg.channel_id
        .say(&ctx, format!("Join role {} added!", role.name))
        .await?;

    Ok(())
}
