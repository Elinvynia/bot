use crate::prelude::*;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};

#[command]
#[only_in(guilds)]
#[num_args(2)]
#[description("Gives a certain amount of money to a user.")]
#[usage("give <user> <amount>")]
#[example("give Elinvynia 1000")]
async fn give(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild_id = msg.guild_id.ok_or_else(|| anyhow!("Guild ID not found."))?;
    let user_arg: String = error_return_ok!(args.single());
    let user_id = none_return_ok!(parse_user(&user_arg, Some(&guild_id), Some(ctx)).await);
    let amount: u64 = error_return_ok!(args.single());

    let author = guild_id.member(&ctx, msg.author.id).await?;
    {
        remove_user_money(guild_id, msg.author.id, amount).await?;
    }

    let member = guild_id.member(&ctx, user_id).await?;
    {
        add_user_money(guild_id, user_id, amount).await?;
    }

    msg.channel_id
        .say(
            &ctx,
            &format!(
                "**{}** has given **{}** **{}**",
                author.display_name(),
                member.display_name(),
                amount
            ),
        )
        .await?;

    Ok(())
}
