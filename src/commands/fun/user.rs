use crate::utils::parse::parse_user;
use chrono::DateTime;
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
    let user_id = if args.len() == 1 {
        match parse_user(
            &args.quoted().current().unwrap().to_string(),
            msg.guild_id.as_ref(),
            Some(&ctx),
        )
        .await
        {
            Some(i) => i,
            None => msg.author.id,
        }
    } else {
        msg.author.id
    };

    let user = user_id.to_user(ctx).await?;
    let mut message = String::from("User found!\n");
    message.push_str(&format!("**Tag:** {}\n", user.tag()));
    message.push_str(&format!("**ID:** {}\n", user.id));
    message.push_str(&format!("**Created At:** {}\n", user.id.created_at()));

    if let Some(guild) = msg.guild(&ctx).await {
        if let Ok(member) = guild.member(&ctx, user_id).await {
            message.push_str(&format!("**Joined At:** {}\n", member.joined_at.unwrap()));
            message.push_str(&format!("**Nickname:** {}\n", member.nick.unwrap_or("None.".into())));

            let mut roles = vec![];
            for role in member.roles {
                roles.push(role.to_role_cached(&ctx).await.unwrap().name)
            }
            message.push_str(&format!("**Roles:** {}\n", roles.join(", ".into())))
        };
    };

    msg.channel_id.say(&ctx, message).await?;

    Ok(())
}
