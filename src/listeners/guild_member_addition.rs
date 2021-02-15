use crate::prelude::*;
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

    let rids = tokio::task::spawn_blocking(move || -> Result<Vec<String>> {
        let conn = connect()?;
        let mut s = conn.prepare("SELECT role_id FROM joinrole WHERE guild_id = ?1")?;
        let q = s.query_and_then(params![guildid.to_string()], |r| r.get(0))?;

        let mut v: Vec<String> = vec![];
        for x in q {
            v.push(x?)
        }

        Ok(v)
    })
    .await;

    let rids = error_return!(rids);
    let rids = error_return!(rids);

    for rid in rids {
        let rid: u64 = match rid.parse() {
            Ok(id) => id,
            Err(_) => return,
        };

        let _ = new_member.add_role(&ctx, rid).await;
    }
}
