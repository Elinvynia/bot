use crate::prelude::*;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};

#[command]
#[only_in(guilds)]
#[owners_only]
#[num_args(2)]
#[description("Sets the amount of money for a user.")]
#[usage("setmoney <user> <amount>")]
#[example("setmoney Elinvynia 1000")]
async fn setmoney(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild_id = msg.guild_id.ok_or(BotError::NoneError)?;
    let user_id = match parse_user(&args.single::<String>()?, Some(&guild_id), Some(&ctx)).await {
        Some(id) => id,
        None => return Ok(()),
    };
    let amount: Money = args.single()?;
    let member = guild_id.member(&ctx, user_id).await?;

    set_user_money(guild_id, user_id, amount).await?;

    msg.channel_id.say(&ctx, &format!("**{}** now has **{}**", member.display_name(), amount)).await?;

    Ok(())
}
