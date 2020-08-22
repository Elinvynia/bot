use crate::{data::error::BotError, db::connect};
use log::info;
use serenity::{collector::*, futures::StreamExt, model::prelude::*, prelude::*};
use sqlx::prelude::*;

pub async fn start_reactions(ctx: &Context) -> Result<(), BotError> {
    let mut conn = connect().await?;

    let mut q = sqlx::query("SELECT * FROM reactionroles").fetch(&mut conn);

    while let Ok(Some(x)) = q.next().await {
        let guild_id: String = x.get("guild_id");
        let guild_id: u64 = guild_id.parse().unwrap();
        let _guild_id = GuildId(guild_id);

        let message_id: String = x.get("message_id");
        let message_id: u64 = message_id.parse().unwrap();
        let message_id = MessageId(message_id);

        let role_id: String = x.get("role_id");
        let role_id: u64 = role_id.parse().unwrap();
        let role_id = RoleId(role_id);

        let reaction_id: String = x.get("reaction_id");
        let reaction_id: u64 = reaction_id.parse().unwrap();
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
                        let uid = a.user_id.unwrap();
                        let guild = a.guild_id.unwrap().to_partial_guild(&http).await.unwrap();
                        let mut member = guild.member(&ctx, uid).await.unwrap();
                        let _ = member.add_role(&http, role_id).await;
                    }
                    ReactionAction::Removed(r) => {
                        let uid = r.user_id.unwrap();
                        let guild = r.guild_id.unwrap().to_partial_guild(&http).await.unwrap();
                        let mut member = guild.member(&ctx, uid).await.unwrap();
                        let _ = member.remove_role(&http, role_id).await;
                    }
                };
            }
        });
    }

    info!("Reaction roles started.");
    Ok(())
}
