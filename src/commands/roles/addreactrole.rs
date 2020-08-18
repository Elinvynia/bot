use crate::{
    db::connect,
    utils::parse::{parse_reaction, parse_rol},
};
use serenity::{
    collector::*,
    framework::standard::{macros::command, Args, CommandResult},
    futures::StreamExt,
    model::prelude::*,
    prelude::*,
};

#[command]
#[only_in(guilds)]
#[num_args(2)]
#[description("Makes the reaction to the message above add the role to a user.")]
#[usage("addreactrole <emoji> <role>")]
#[example("addreactrole :heart: Admin")]
async fn addreactrole(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut conn = connect().await?;
    let x = msg
        .channel(ctx)
        .await
        .unwrap()
        .guild()
        .unwrap()
        .messages(ctx, |builder| builder.before(msg.id).limit(1))
        .await?;
    if x.is_empty() {
        return Ok(());
    };

    let gid = msg.guild_id.unwrap();

    let reaction = match parse_reaction(&args.single::<String>().unwrap(), &gid, &ctx).await {
        Some(r) => r,
        None => return Ok(()),
    };

    let role = match parse_rol(&args.single::<String>().unwrap(), Some(&gid), Some(&ctx)).await {
        Some(rid) => match rid.to_role_cached(&ctx.cache).await {
            Some(r) => r,
            None => return Ok(()),
        },
        None => return Ok(()),
    };

    let reactionid = reaction.id;
    let parent_msg = x.get(0).unwrap().clone();
    parent_msg.react(&ctx, reaction).await?;

    let roleid = role.id;
    let collector = ReactionCollectorBuilder::new(&ctx)
        .message_id(parent_msg.id)
        .removed(true)
        .filter(move |reaction| match reaction.as_ref().emoji {
            ReactionType::Custom { id, .. } => id == reactionid,
            _ => false,
        })
        .await;

    let http = &ctx.http;

    let _ = msg.delete(&ctx).await;

    sqlx::query("INSERT INTO reactionroles (guild_id, message_id, role_id, reaction_id) values (?1, ?2, ?3, ?4)")
        .bind(gid.to_string())
        .bind(parent_msg.id.to_string())
        .bind(roleid.to_string())
        .bind(reactionid.to_string())
        .execute(&mut conn)
        .await?;

    let _: Vec<_> = collector
        .then(|action| async move {
            match action.as_ref() {
                ReactionAction::Added(a) => {
                    let uid = a.user_id.unwrap();
                    let guild = a.guild_id.unwrap().to_partial_guild(&http).await.unwrap();
                    let mut member = guild.member(&http, uid).await.unwrap();
                    let _ = member.add_role(&http, roleid).await;
                }
                ReactionAction::Removed(r) => {
                    let uid = r.user_id.unwrap();
                    let guild = r.guild_id.unwrap().to_partial_guild(&http).await.unwrap();
                    let mut member = guild.member(&http, uid).await.unwrap();
                    let _ = member.remove_role(&http, roleid).await;
                }
            };
        })
        .collect()
        .await;

    Ok(())
}
