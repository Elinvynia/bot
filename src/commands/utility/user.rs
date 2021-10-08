use crate::prelude::*;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};

#[command]
#[min_args(0)]
#[max_args(1)]
#[description("Retrieves information about a user.")]
#[usage("user <optional: person>")]
#[example("user Elinvynia")]
async fn user(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let user_id;
    if !args.is_empty() && msg.guild_id.is_some() {
        let name: String = error_return_ok!(args.single());
        let gid = msg.guild_id.ok_or_else(|| anyhow!("Guild ID not found."))?;

        user_id = none_return_ok!(parse_user(&name, Some(&gid), Some(ctx)).await);
    } else {
        user_id = msg.author.id
    }

    let user = user_id.to_user(ctx).await?;
    let mut message = String::from("User found!\n");
    message += &format!("**Tag:** {}\n", user.tag());
    message += &format!("**ID:** {}\n", user.id);
    message += &format!("**Created At:** {}\n", user.id.created_at().format("%F %T"));

    if let Some(guild) = msg.guild(&ctx).await {
        if let Ok(member) = guild.member(&ctx, user_id).await {
            message += &format!(
                "**Joined At:** {}\n",
                member.joined_at.ok_or("Member join time not found.")?.format("%F %T")
            );
            message += &format!("**Nickname:** {}\n", member.nick.unwrap_or_else(|| "None.".into()));

            let mut roles = vec![];
            for role in member.roles {
                roles.push(
                    role.to_role_cached(&ctx)
                        .await
                        .ok_or_else(|| anyhow!("Role not found in cache."))?
                        .name,
                )
            }
            message += &format!("**Roles:** {}\n", roles.join(", "))
        };
    };

    msg.channel_id.say(&ctx, message).await?;

    Ok(())
}
