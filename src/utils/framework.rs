use crate::data::cache::{BotId, BotOwners, DefaultPrefix, GuildPrefixes};
use crate::db::prefix::get_prefix;
use log::error;
use serenity::{
    framework::standard::{CommandError, DispatchError},
    model::prelude::*,
    prelude::*,
};

//Sends non-command DMs from regular users to the bot owners.
pub fn log_dm(ctx: &mut Context, message: &Message) {
    if message.guild_id.is_some() {
        return;
    }

    let data = ctx.data.read();

    if &message.author.id == data.get::<BotId>().unwrap() {
        return;
    }

    let owners = data.get::<BotOwners>().unwrap();
    for x in owners.iter() {
        if &message.author.id == x {
            continue;
        }
        let _ = x
            .to_user(&ctx)
            .unwrap()
            .create_dm_channel(&ctx)
            .unwrap()
            .say(
                &ctx.http,
                format!("DM from {}:\n{}", &message.author, &message.content),
            );
    }
}

//Generic handling of common user errors.
pub fn dispatch_error(context: &mut Context, msg: &Message, error: DispatchError) {
    match error {
        DispatchError::NotEnoughArguments { min, given } => {
            let _ = msg.channel_id.say(
                &context.http,
                format!("Need {} arguments, but only got {}.", min, given),
            );
        }
        DispatchError::TooManyArguments { max, given } => {
            let _ = msg.channel_id.say(
                &context.http,
                format!("Max arguments allowed is {}, but got {}.", max, given),
            );
        }
        _ => error!("Unhandled dispatch error."),
    }
}

//Logs every command that errored, should only be used for bot failures and not user failures.
pub fn after(_ctx: &mut Context, _msg: &Message, _cmd_name: &str, error: Result<(), CommandError>) {
    if let Err(why) = error {
        error!("{:?}", why);
    }
}

//Allows the use of a per-guild prefix with a default one set using the config file.
pub fn dynamic_prefix(ctx: &mut Context, msg: &Message) -> Option<String> {
    let default_prefix;
    {
        let data = ctx.data.read();
        default_prefix = data.get::<DefaultPrefix>().unwrap().clone();
    }

    if msg.is_private() {
        return Some(default_prefix.to_string());
    }

    if msg.guild_id.is_none() {
        return Some(default_prefix.to_string());
    }

    let guildid = msg.guild_id.unwrap();

    {
        let data = ctx.data.read();
        let prefixes = data.get::<GuildPrefixes>().unwrap();
        if let Some(x) = prefixes.get(&guildid) {
            return Some(x.to_string());
        }
    }

    return Some(
        get_prefix(&guildid, &ctx)
            .map_or_else(|_| default_prefix, |pref| pref)
            .to_string(),
    );
}
