use crate::prelude::*;
use serenity::{model::prelude::*, prelude::*};
use sqlx::prelude::*;

pub async fn guild_member_addition(ctx: Context, guildid: GuildId, mut new_member: Member) {
    if check_log_type(LogType::UserJoined, guildid).await.is_err() {
        return;
    }

    let log_channel = match get_log_channel(guildid).await {
        Ok(c) => c,
        Err(_) => return,
    };

    let user = new_member.clone().user;
    let avatar = user.face().replace("size=1024", "size=128");
    let mut msg = String::from("**User Joined**\n");
    msg += &format!("ID: {}\n", user.id);
    msg += &format!("Tag: {}\n", user.tag());
    msg += &format!("Ping: {}\n", user.mention());

    let _ = log_channel
        .send_message(&ctx.http, |message| {
            message.content(msg);
            message.add_file(&avatar[..]);
            message
        })
        .await;

    if let Ok(mut conn) = connect().await {
        let mut q = sqlx::query("SELECT * FROM joinrole WHERE guild_id = ?1")
            .bind(guildid.to_string())
            .fetch(&mut conn);

        if let Ok(Some(row)) = q.next().await {
            let role_id: String = row.get("role_id");
            let rid: u64 = match role_id.parse() {
                Ok(id) => id,
                Err(_) => return,
            };

            let _ = new_member.add_role(&ctx, rid).await;
        }
    }
}
