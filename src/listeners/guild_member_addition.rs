use crate::prelude::*;
use serenity::futures::TryStreamExt;
use serenity::{model::prelude::*, prelude::*};

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

    let mut conn = match connect().await {
        Ok(c) => c,
        Err(_) => return,
    };

    let gid = guildid.to_string();
    let mut result = sqlx::query!("SELECT role_id FROM joinrole WHERE guild_id = ?1", gid).fetch(&mut conn);

    while let Ok(Some(row)) = result.try_next().await {
        let _ = new_member.add_role(&ctx, row.role_id.parse::<u64>().unwrap()).await;
    }
}
