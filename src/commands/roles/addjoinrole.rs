use crate::{
    db::connect,
    utils::parse::parse_rol,
};
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
    let gid = msg.guild_id.unwrap();

    let role = match parse_rol(&args.single::<String>().unwrap(), Some(&gid), Some(&ctx)).await {
        Some(rid) => match rid.to_role_cached(&ctx.cache).await {
            Some(r) => r,
            None => return Ok(()),
        },
        None => return Ok(()),
    };

    sqlx::query("INSERT INTO joinrole (guild_id, role_id) values (?1, ?2)")
        .bind(gid.to_string())
        .bind(role.id.to_string())
        .execute(&mut conn)
        .await?;

    msg.reply(&ctx, format!("Join role {} added!", role.name)).await?;

    Ok(())
}
