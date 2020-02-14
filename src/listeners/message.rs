use crate::data::error::BotError;
use crate::db::{get_db, leaderboard::get_user_channel_score};
use log::error;
use serenity::{model::prelude::*, prelude::*};

pub fn message(_: Context, new_message: Message) {
    let guild_id = match new_message.guild_id {
        Some(g) => g,
        None => return,
    };

    let channel_id = new_message.channel_id;

    let conn = match get_db() {
        Ok(c) => c,
        Err(e) => {
            error!("{:?}", e);
            return;
        }
    };

    if new_message.author.bot {
        return;
    }

    let user_id = new_message.author.id;

    match get_user_channel_score(&guild_id, &channel_id, &user_id) {
        Ok(_) => {
            let _ = conn.execute(
                "UPDATE leaderboard SET points = points + 1 WHERE guild_id == ?1 AND channel_id == ?2 AND user_id == ?3;",
                &[
                    &guild_id.as_u64().to_string(),
                    &channel_id.as_u64().to_string(),
                    &user_id.as_u64().to_string(),
                ],
            );
        }
        Err(BotError::CustomError(e)) if e == "No record yet." => {
            let _ = conn.execute(
                "INSERT INTO leaderboard (guild_id, channel_id, user_id, points) values (?1, ?2, ?3, 1);",
                &[
                    &guild_id.as_u64().to_string(),
                    &channel_id.as_u64().to_string(),
                    &user_id.as_u64().to_string(),
                ],
            );
        }
        Err(_) => {
            return;
        }
    }
}
