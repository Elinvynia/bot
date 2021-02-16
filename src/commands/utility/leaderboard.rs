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
#[description("Retrieves the leaderboard (of a channel/user).")]
#[usage("leaderboard <optional: channel|user>")]
#[example("leaderboard #general")]
async fn leaderboard(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.len() == 1 {
        let guild_id = msg.guild_id.ok_or_else(|| anyhow!("Guild ID not found."))?;
        if let Some(uid) = parse_user(
            &args
                .quoted()
                .current()
                .ok_or_else(|| anyhow!("Argument not found."))?
                .to_string(),
            Some(&guild_id),
            Some(&ctx),
        )
        .await
        {
            return parse_user_score(ctx, msg, uid).await;
        };
        if let Some(chid) = parse_chan(
            &args
                .quoted()
                .current()
                .ok_or_else(|| anyhow!("Argument not found."))?
                .to_string(),
            Some(&guild_id),
            Some(&ctx),
        )
        .await
        {
            return parse_channel_score(ctx, msg, chid).await;
        };
        Ok(())
    } else {
        return parse_total_score(ctx, msg).await;
    }
}

async fn parse_user_score(ctx: &Context, msg: &Message, user_id: UserId) -> CommandResult {
    let guild_id = msg.guild_id.ok_or_else(|| anyhow!("Guild ID not found."))?;
    let rows = get_single_scores(guild_id, user_id).await?;
    let member = guild_id.member(ctx, user_id).await?;

    let mut result = String::new();
    for row in rows {
        let cid = match parse_chan(&row.channel_id, Some(&guild_id), Some(&ctx)).await {
            Some(id) => id,
            None => continue,
        };
        let channel = match cid.to_channel(&ctx).await {
            Ok(c) => c,
            Err(_) => continue,
        };
        result += &format!("{} - {}\n", channel, row.points)[..];
    }

    msg.channel_id
        .say(&ctx, format!("**Leaderboard** - {}\n{}", member.display_name(), result))
        .await?;

    Ok(())
}

async fn parse_channel_score(ctx: &Context, msg: &Message, channel_id: ChannelId) -> CommandResult {
    let guild_id = msg.guild_id.ok_or_else(|| anyhow!("Guild ID not found."))?;
    let channel = channel_id.to_channel_cached(&ctx).await.ok_or("Channel not found.")?;
    let rows = get_channel_scores(guild_id, channel_id).await?;
    let mut result = String::new();

    let mut processed = 0;
    for x in rows.iter() {
        if processed == 10 {
            break;
        };
        let id = x.user_id.parse::<u64>()?;
        let member = match guild_id.member(ctx, id).await {
            Ok(m) => m,
            Err(_) => continue,
        };
        result += &format!("{}. {} - {}\n", processed + 1, member.display_name(), x.points)[..];
        processed += 1;
    }

    msg.channel_id
        .say(&ctx, format!("**Leaderboard** - {} - Top 10\n{}", channel, result))
        .await?;

    Ok(())
}

async fn parse_total_score(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild_id.ok_or_else(|| anyhow!("Guild ID not found."))?;
    let rows = get_user_scores(guild_id).await?;
    let mut result = String::new();

    let mut user_found = false;
    let mut processed = 0;
    for x in rows.iter() {
        if processed >= 10 && user_found {
            break;
        };
        let id = x.user_id.parse::<u64>()?;
        let member = match guild_id.member(ctx, id).await {
            Ok(m) => m,
            Err(_) => continue,
        };
        if processed < 10 {
            result += &format!("{}. {} - {}\n", processed + 1, member.display_name(), x.points)[..];
            if member.user == msg.author {
                user_found = true;
            };
        } else if member.user == msg.author {
            result += "...\n";
            result += &format!("{}. {} - {}\n", processed + 1, member.display_name(), x.points)[..];
            break;
        }
        processed += 1;
    }

    msg.channel_id
        .say(&ctx, format!("**Leaderboard** - Top 10\n{}", result))
        .await?;
    Ok(())
}
