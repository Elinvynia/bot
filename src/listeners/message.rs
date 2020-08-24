use crate::data::error::BotError;
use crate::db::{connect, leaderboard::get_user_channel_score};
use log::error;
use serenity::model::prelude::*;

pub async fn message(new_message: Message) {
    let guild_id = match new_message.guild_id {
        Some(g) => g,
        None => return,
    };

    let channel_id = new_message.channel_id;

    let mut conn = match connect().await {
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

    match get_user_channel_score(guild_id, channel_id, user_id).await {
        Ok(_) => {
            let _ = sqlx::query("UPDATE leaderboard SET points = points + 1 WHERE guild_id == ?1 AND channel_id == ?2 AND user_id == ?3;")
                .bind(&guild_id.as_u64().to_string())
                .bind(&channel_id.as_u64().to_string())
                .bind(&user_id.as_u64().to_string())
                .execute(&mut conn)
                .await;
        }
        Err(BotError::CustomError(e)) if e == "No record yet." => {
            let _ =
                sqlx::query("INSERT INTO leaderboard (guild_id, channel_id, user_id, points) values (?1, ?2, ?3, 1);")
                    .bind(&guild_id.as_u64().to_string())
                    .bind(&channel_id.as_u64().to_string())
                    .bind(&user_id.as_u64().to_string())
                    .execute(&mut conn)
                    .await;
        }
        Err(_) => {}
    }
}
