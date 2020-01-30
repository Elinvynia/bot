use crate::db::*;
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::*,
    prelude::*,
};
use std::error::Error;

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

    let _ = msg.channel_id.say(&ctx, format!("**Leaderboard** - Top 10\n{}", result));

    Ok(())
}

#[derive(Debug)]
struct LeaderboardEntry {
    user_id: String,
    points: i64,
}

fn get_user_scores(guildid: &GuildId) -> Result<Vec<LeaderboardEntry>, Box<dyn Error>> {
    let guild_id = guildid.as_u64().to_string();
    let conn = get_db()?;
    let mut statement = conn.prepare("SELECT user_id, points FROM leaderboard WHERE guild_id == ?1 ORDER BY points DESC LIMIT 10;")?;
    let result_iter = statement.query_map(&[&guild_id], |row| {
        Ok(LeaderboardEntry {
            user_id: row.get(0)?,
            points: row.get(1)?,
        })
    })?;

    let mut result = Vec::new();
    for x in result_iter {
        result.push(x?);
    }

    Ok(result)
}
