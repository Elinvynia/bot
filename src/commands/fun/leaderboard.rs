use crate::db::get_user_scores;
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::*,
    prelude::*,
};

#[command]
#[only_in(guilds)]
fn leaderboard(ctx: &mut Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();
    let rows = match get_user_scores(&guild_id) {
        Ok(r) => r,
        Err(_) => return Ok(()),
    };

    let mut result = "".to_string();
    for x in rows.iter() {
        result.push_str(
            &format!(
                "{} - {}\n",
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

    Ok(())
}
