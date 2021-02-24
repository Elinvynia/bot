use crate::prelude::*;
use serenity::futures::{StreamExt, TryStreamExt};
use serenity::{collector::*, model::prelude::*, prelude::*};

pub async fn start_reactions(ctx: &Context) -> Result<()> {
    let mut conn = connect().await?;

    let mut rows = sqlx::query!("SELECT message_id, role_id, reaction_id FROM reactionroles").fetch(&mut conn);

    while let Ok(Some(row)) = rows.try_next().await {
        let message_id: u64 = match row.message_id.parse() {
            Ok(id) => id,
            Err(_) => continue,
        };
        let message_id = MessageId(message_id);

        let role_id: u64 = match row.role_id.parse() {
            Ok(id) => id,
            Err(_) => continue,
        };
        let role_id = RoleId(role_id);

        let reaction_id: u64 = match row.reaction_id.parse() {
            Ok(id) => id,
            Err(_) => continue,
        };
        let reaction_id = EmojiId(reaction_id);

        let mut collector = ReactionCollectorBuilder::new(&ctx)
            .message_id(message_id)
            .removed(true)
            .filter(move |reaction| match reaction.as_ref().emoji {
                ReactionType::Custom { id, .. } => id == reaction_id,
                _ => false,
            })
            .await;

        let ctx = ctx.clone();
        tokio::spawn(async move {
            let http = &ctx.http;
            while let Some(event) = collector.next().await {
                match event.as_ref() {
                    ReactionAction::Added(a) => {
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
                        let _ = member.add_role(&http, role_id).await;
                    }
                    ReactionAction::Removed(r) => {
                        let uid = match r.user_id {
                            Some(id) => id,
                            None => continue,
                        };
                        let gid = match r.guild_id {
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
                        let _ = member.remove_role(&http, role_id).await;
                    }
                };
            }
        });
    }

    info!("Reaction roles started.");
    Ok(())
}
