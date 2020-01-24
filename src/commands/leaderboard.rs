use crate::util::*;
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
        Err(_) => return Ok(())
    };

    let mut result = "".to_string();
    for (id, points) in rows.iter() {
        result.push_str(&format!("{} - {}\n", &ctx.http.get_member(*msg.guild_id.unwrap().as_u64(), id.parse::<u64>().unwrap()).unwrap().user.read().name, points)[..])
    };

    let _ = msg.channel_id
        .send_message(&ctx, |m| {
            m.embed(|e| {
                e.title("Leaderboard");

                e.field("Top 10:", result, false);

                e
            })
    });

    Ok(())

}