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
#[description("Makes the reaction to the message above no longer add the role to a user.")]
#[usage("removereactrole <emoji>")]
#[example("removereactrole :heart:")]
async fn removereactrole(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut conn = connect().await?;
    let gid = msg.guild_id.ok_or_else(|| anyhow!("Guild ID not found."))?;

    let reaction = match parse_reaction(&args.single::<String>()?, &gid, &ctx).await {
        Some(r) => r,
        None => return Ok(()),
    };

    let (rid, gid) = (reaction.id.to_string(), gid.to_string());
    sqlx::query!(
        "DELETE FROM reactionroles WHERE reaction_id = ?1 AND guild_id = ?2",
        rid,
        gid
    )
    .execute(&mut conn)
    .await?;

    msg.channel_id.say(&ctx, "Reaction role removed!").await?;

    Ok(())
}
