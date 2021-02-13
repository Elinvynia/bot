use crate::prelude::*;
use log::info;
use serenity::{
    collector::*,
    futures::{StreamExt, TryStreamExt},
    model::prelude::*,
    prelude::*,
};
use sqlx::prelude::*;

pub async fn start_reactions(ctx: &Context) -> Result<(), BotError> {
    let mut conn = connect().await?;

    let mut q = sqlx::query("SELECT * FROM reactionroles").fetch(&mut conn);

    while let Ok(Some(x)) = q.try_next().await {
        let message_id: String = x.get("message_id");
        let message_id: u64 = match message_id.parse() {
            Ok(id) => id,
            Err(_) => continue,
        };
        let message_id = MessageId(message_id);

        let role_id: String = x.get("role_id");
        let role_id: u64 = match role_id.parse() {
            Ok(id) => id,
            Err(_) => continue,
        };
        let role_id = RoleId(role_id);

        let reaction_id: String = x.get("reaction_id");
        let reaction_id: u64 = match reaction_id.parse() {
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
