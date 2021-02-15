use crate::prelude::*;
use serenity::{
    collector::*,
    futures::StreamExt,
    model::prelude::*,
    prelude::*,
};

pub struct ReactionResponse {
    pub message_id: String,
    pub role_id: String,
    pub reaction_id: String,
}

pub async fn start_reactions(ctx: &Context) -> Result<()> {
    let conn = connect()?;

    let mut rows = vec![];

    sql_block!({
        let mut s = conn.prepare("SELECT message_id, role_id, reaction_id FROM reactionroles")?;
        let q = s.query_map(NO_PARAMS, |r| {
            Ok(ReactionResponse{
                message_id: r.get(0)?,
                role_id: r.get(1)?,
                reaction_id: r.get(2)?,
            })
        })?;

        for x in q {
            rows.push(x?)
        }
    })?;



   for row in rows {
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
