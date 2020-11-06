use crate::prelude::*;
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
async fn leaderboard(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if args.len() == 1 {
        parse_channel_score(ctx, msg, args).await
    }
    else {
        parse_total_score(ctx, msg).await
    }
}


async fn parse_channel_score(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild_id = msg.guild_id.ok_or(BotError::NoneError)?;
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
    let mut result = String::new();

    let mut processed = 0;
    for x in rows.iter() {
        if processed == 10 {
            break
        };
        let id = x.user_id.parse::<u64>()?;
        let user = match guild_id.member(ctx, id).await {
            Ok(m) => m.user.clone(),
            Err(_) => continue,
        };
        result += &format!("{}. {} - {}\n", processed + 1, user.name, x.points)[..];
        processed += 1;
    }

    msg.channel_id
        .say(&ctx, format!("**Leaderboard** - {} - Top 10\n{}", channel, result))
        .await?;

    Ok(())
}

async fn parse_total_score(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild_id.ok_or(BotError::NoneError)?;
    let rows = get_user_total_scores(guild_id).await?;
    let mut result = String::new();

    let mut processed = 0;
    for x in rows.iter() {
        if processed == 10 {
            break
        };
        let id = x.user_id.parse::<u64>()?;
        let user = match guild_id.member(ctx, id).await {
            Ok(m) => m.user.clone(),
            Err(_) => continue,
        };
        result += &format!("{}. {} - {}\n", processed + 1, user.name, x.points)[..];
        processed += 1;
    }

    msg.channel_id
        .say(&ctx, format!("**Leaderboard** - Top 10\n{}", result))
        .await?;
        Ok(())
}
