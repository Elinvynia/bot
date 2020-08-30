use crate::{
    data::error::BotError,
    db::leaderboard::{get_user_channel_scores, get_user_total_scores},
    utils::parse::parse_chan,
};
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};

#[command]
#[only_in(guilds)]
#[min_args(0)]
#[max_args(1)]
#[description("Retrieves the leaderboard of a channel.")]
#[usage("leaderboard <optional: channel>")]
#[example("leaderboard #general")]
async fn leaderboard(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild_id = msg.guild_id.ok_or(BotError::NoneError)?;

    if args.len() == 1 {
        let channel_id = match parse_chan(
            &args.quoted().current().ok_or(BotError::NoneError)?.to_string(),
            Some(&guild_id),
            Some(&ctx),
        )
        .await
        {
            Some(c) => c,
            None => return Ok(()),
        };
        let channel = channel_id.to_channel_cached(&ctx).await.ok_or("Channel not found.")?;
        let rows = get_user_channel_scores(guild_id, channel_id).await?;
        let mut result = "".to_string();
        for (i, x) in rows.iter().enumerate() {
            let id = x.user_id.parse::<u64>()?;
            let user = match guild_id.member(ctx, id).await {
                Ok(m) => m.user.clone(),
                Err(_) => ctx.http.get_user(id).await?,
            };
            result.push_str(&format!("{}. {} - {}\n", i + 1, user.name, x.points)[..])
        }

        msg.channel_id
            .say(&ctx, format!("**Leaderboard** - {} - Top 10\n{}", channel, result))
            .await?;
    } else {
        let rows = get_user_total_scores(guild_id).await?;
        let mut result = "".to_string();
        for (i, x) in rows.iter().enumerate() {
            let id = x.user_id.parse::<u64>()?;
            let user = match guild_id.member(ctx, id).await {
                Ok(m) => m.user.clone(),
                Err(_) => ctx.http.get_user(id).await?,
            };
            result.push_str(&format!("{}. {} - {}\n", i + 1, user.name, x.points)[..])
        }

        msg.channel_id
            .say(&ctx, format!("**Leaderboard** - Top 10\n{}", result))
            .await?;
    }

    Ok(())
}
