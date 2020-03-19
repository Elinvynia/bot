use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::*,
    prelude::*,
};

#[command]
#[only_in(guilds)]
async fn guild(ctx: &mut Context, msg: &Message) -> CommandResult {
    let g = match msg.guild_id.unwrap().to_guild_cached(&ctx).await {
        Some(g) => g,
        None => {
            msg.channel_id
                .say(&ctx.http, "Guild not found in cache.")
                .await?;
            return Ok(());
        }
    };

    let guild = g.read().await;
    let owner = guild
        .owner_id
        .to_user(&ctx)
        .await
        .map_or("Owner information not found.".to_string(), |user| {
            user.tag()
        });

    msg.channel_id
        .say(
            &ctx.http,
            format!(
                "**{}**\n**ID:** {}\n**Owner:** {}\n**Region:** {}\n",
                guild.name, guild.id, owner, guild.region
            ),
        )
        .await?;

    Ok(())
}
