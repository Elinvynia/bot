use crate::prelude::*;
use rand::prelude::*;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};

#[command]
#[only_in(guilds)]
#[num_args(2)]
#[description("Bets a certain amount of currency and rolls a dice. Rolling over 66 yields x2 of your currency, over 90 - x4 and 100 x10.")]
#[usage("flip <amount> <heads/tails>")]
#[example("flip 100 heads")]
async fn flip(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guildid = msg.guild_id.ok_or(BotError::NoneError)?;
    let userid = msg.author.id;

    let money = get_user_money(guildid, userid).await?;
    let bet: Money = error_return_ok!(args.single());

    let side = match &args.single::<String>()?[..] {
        "h" | "heads" => true,
        "t" | "tails" => false,
        _ => return Ok(()),
    };

    if *bet == 0 {
        return Ok(());
    };

    if bet > money {
        msg.channel_id.say(&ctx, "You don't have enough money!").await?;
        return Ok(());
    };

    let roll: bool = rand::thread_rng().gen();
    let new_amount;

    if roll == side {
        new_amount = bet * Money(2);
    } else {
        new_amount = money - bet;
    };

    msg.channel_id
        .say(&ctx, &format!("Side: {}\nYou now have: {}", roll, new_amount))
        .await?;

    Ok(())
}
