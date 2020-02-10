use crate::db::{get_user_channel_scores, get_user_total_scores};
use crate::util::parse_chan;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};

#[command]
#[only_in(guilds)]
#[min_args(0)]
#[max_args(1)]
fn leaderboard(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();

    if args.len() == 1 {
        let channel_id = parse_chan(
            &args.quoted().current().unwrap().to_string(),
            Some(&guild_id),
            Some(&ctx),
        )
        .ok_or("Failed to parse channel.")?;
        let channel = channel_id
            .to_channel_cached(&ctx)
            .ok_or("Channel not found.")?;
        let rows = get_user_channel_scores(&guild_id, &channel_id)?;
        let mut result = "".to_string();
        for (i, x) in rows.iter().enumerate() {
            result.push_str(
                &format!(
                    "{}. {} - {}\n",
                    i + 1,
                    msg.guild_id
                        .unwrap()
                        .member(&ctx, x.user_id.parse::<u64>().unwrap())
                        .unwrap()
                        .user
                        .read()
                        .name,
                    x.points
                )[..],
            )
        }

        let _ = msg.channel_id.say(
            &ctx,
            format!("**Leaderboard** - {} - Top 10\n{}", channel, result),
        );
    } else {
        let rows = get_user_total_scores(&guild_id)?;
        let mut result = "".to_string();
        for (i, x) in rows.iter().enumerate() {
            result.push_str(
                &format!(
                    "{}. {} - {}\n",
                    i + 1,
                    msg.guild_id
                        .unwrap()
                        .member(&ctx, x.user_id.parse::<u64>().unwrap())
                        .unwrap()
                        .user
                        .read()
                        .name,
                    x.points
                )[..],
            )
        }

        let _ = msg
            .channel_id
            .say(&ctx, format!("**Leaderboard** - Top 10\n{}", result));
    }

    Ok(())
}
