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
    let guildid = msg.guild_id.ok_or(anyhow!("Guild ID not found."))?;
    let userid = msg.author.id;

    let money = get_user_money(guildid, userid).await?;
    let bet: i64 = error_return_ok!(args.single());

    let coin = match &args.single::<String>()?[..] {
        "h" | "heads" => Coin::Heads,
        "t" | "tails" => Coin::Tails,
        _ => return Ok(()),
    };

    if bet == 0 {
        return Ok(());
    };

    if bet > money {
        msg.channel_id.say(&ctx, "You don't have enough money!").await?;
        return Ok(());
    };

    let roll: bool = rand::thread_rng().gen();
    let roll_coin: bool = coin.into();

    let new_amount;
    if roll == roll_coin {
        new_amount = money + (bet * 2);
    } else {
        new_amount = money - bet;
    };

    set_user_money(guildid, userid, new_amount).await?;

    msg.channel_id
        .say(&ctx, &format!("Your Side: {}\nRolled: {}\nYou now have: {}", coin, roll_coin, new_amount))
        .await?;

    Ok(())
}

#[derive(Debug, Copy, Clone)]
enum Coin {
    Heads,
    Tails,
}

impl From<Coin> for bool {
    fn from(coin: Coin) -> Self {
        match coin {
            Coin::Heads => true,
            Coin::Tails => true,
        }
    }
}

impl std::fmt::Display for Coin {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match *self {
            Coin::Heads => write!(fmt, "Heads"),
            Coin::Tails => write!(fmt, "Tails"),
        }
    }
}
