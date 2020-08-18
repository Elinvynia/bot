use crate::{db::connect, utils::parse::parse_reaction};
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};

#[command]
#[only_in(guilds)]
#[num_args(1)]
#[description("Makes the reaction to the message above no longer add the role to a user.")]
#[usage("removereactrole <emoji>")]
#[example("removereactrole :heart:")]
async fn removereactrole(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut conn = connect().await?;

    let gid = msg.guild_id.unwrap();

    let reaction = match parse_reaction(&args.single::<String>().unwrap(), &gid, &ctx).await {
        Some(r) => r,
        None => return Ok(()),
    };

    sqlx::query("DELETE FROM reactionroles WHERE reaction_id = ?1 AND guild_id = ?2")
        .bind(reaction.id.to_string())
        .bind(gid.to_string())
        .execute(&mut conn)
        .await?;

    Ok(())
}
