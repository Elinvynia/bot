use crate::prelude::*;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};
use rand::prelude::*;

#[command]
#[only_in(guilds)]
#[num_args(1)]
#[description("Bets a certain amount of currency and rolls a dice. Rolling over 66 yields x2 of your currency, over 90 - x4 and 100 x10.")]
#[usage("betroll <amount>")]
#[example("betroll 100")]
async fn betroll(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guildid = msg.guild_id.ok_or(BotError::NoneError)?;
    let userid = msg.author.id;
    let money = get_user_money(guildid, userid).await?;
    let bet: Money = match args.single() {
        Ok(m) => m,
        Err(_) => return Ok(()),
    };
    if *bet == 0 {
        return Ok(())
    };
    if bet > money {
        msg.channel_id.say(&ctx, "You don't have enough money!").await?;
        return Ok(())
    };

    let roll: u32 = rand::thread_rng().gen_range(1, 101);
    let new_amount;

    if roll == 100 {
        new_amount = money + (bet * Money(10));
        set_user_money(guildid, userid, new_amount).await?;
    } else if roll > 90 {
        new_amount = money + (bet * Money(4));
        set_user_money(guildid, userid, new_amount).await?;
    } else if roll > 66 {
        new_amount = money + (bet * Money(2));
        set_user_money(guildid, userid, new_amount).await?;
    } else {
        new_amount = money - bet;
        set_user_money(guildid, userid, new_amount).await?;
    }

    msg.channel_id.say(&ctx, &format!("Roll: {}\nYou now have: {}", roll, new_amount)).await?;

    Ok(())
}
