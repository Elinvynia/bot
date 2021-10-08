use crate::prelude::*;
use serenity::{
    collector::*,
    framework::standard::{macros::command, Args, CommandResult},
    futures::StreamExt,
    model::prelude::*,
    prelude::*,
};

#[command]
#[only_in(guilds)]
#[owners_only]
#[num_args(2)]
#[description("Makes the reaction to the message above add the role to a user.")]
#[usage("addreactrole <emoji> <role>")]
#[example("addreactrole :heart: Admin")]
async fn addreactrole(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut conn = connect().await?;
    let x = msg
        .channel(ctx)
        .await
        .ok_or_else(|| anyhow!("Channel not found."))?
        .guild()
        .ok_or_else(|| anyhow!("Guild not found."))?
        .messages(ctx, |builder| builder.before(msg.id).limit(1))
        .await?;

    if x.is_empty() {
        return Ok(());
    };

    let gid = msg.guild_id.ok_or_else(|| anyhow!("Guild ID not found."))?;

    let reaction = match parse_reaction(&args.single::<String>()?, &gid, ctx).await {
        Some(r) => r,
        None => return Ok(()),
    };

    let role = match parse_rol(&args.single::<String>()?, Some(&gid), Some(ctx)).await {
        Some(rid) => match rid.to_role_cached(&ctx.cache).await {
            Some(r) => r,
            None => return Ok(()),
        },
        None => return Ok(()),
    };

    let reactionid = reaction.id;
    let parent_msg = x.get(0).ok_or_else(|| anyhow!("Parent message not found."))?.clone();
    parent_msg.react(&ctx, reaction).await?;

    let role_id = role.id;
    let mut collector = ReactionCollectorBuilder::new(&ctx)
        .message_id(parent_msg.id)
        .removed(true)
        .filter(move |reaction| match reaction.as_ref().emoji {
            ReactionType::Custom { id, .. } => id == reactionid,
            _ => false,
        })
        .await;

    msg.delete(&ctx).await?;

    let (gid, msgid, rid, reaid) = (
        gid.to_string(),
        parent_msg.id.to_string(),
        role_id.to_string(),
        reactionid.to_string(),
    );
    sqlx::query!(
        "INSERT INTO reactionroles (guild_id, message_id, role_id, reaction_id) values (?1, ?2, ?3, ?4)",
        gid,
        msgid,
        rid,
        reaid
    )
    .execute(&mut conn)
    .await?;

    let ctx = ctx.clone();
    tokio::spawn(async move {
        let http = &ctx.http;
        while let Some(event) = collector.next().await {
            if let ReactionAction::Added(a) = event.as_ref() {
                let uid = match a.user_id {
                    Some(id) => id,
                    None => continue,
                };
                let gid = match a.guild_id {
                    Some(id) => id,
                    None => continue,
                };
                let guild = match gid.to_partial_guild(&http).await {
                    Ok(g) => g,
                    Err(_) => continue,
                };
                let mut member = match guild.member(&ctx, uid).await {
                    Ok(m) => m,
                    Err(_) => continue,
                };
                let roles = match member.roles(&ctx).await {
                    Some(r) => r,
                    None => continue,
                };
                if roles.iter().any(|role| role.id == role_id) {
                    let _ = member.remove_role(&http, role_id).await;
                } else {
                    let _ = member.add_role(&http, role_id).await;
                }
                let _ = a.delete(&ctx.http).await;
            };
        }
    });

    Ok(())
}
