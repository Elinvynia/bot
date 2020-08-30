use crate::data::cache::{BotId, BotOwners, DefaultPrefix, GuildPrefixes};
use crate::db::prefix::get_prefix;
use log::error;
use serenity::{
    framework::standard::{macros::hook, CommandError, DispatchError},
    model::prelude::*,
    prelude::*,
};

//Sends non-command DMs from regular users to the bot owners.
#[hook]
pub async fn log_dm(ctx: &Context, message: &Message) {
    if message.guild_id.is_some() {
        return;
    }

    let data = ctx.data.read().await;
    let bot_id = match data.get::<BotId>() {
        Some(id) => id,
        None => return,
    };

    if &message.author.id == bot_id {
        return;
    };

    let owner_ids = match data.get::<BotOwners>() {
        Some(o) => o,
        None => return,
    };

    for owner_id in owner_ids.iter() {
        if &message.author.id == owner_id {
            continue;
        }
        if let Ok(user) = owner_id.to_user(ctx).await {
            if let Ok(chan) = user.create_dm_channel(ctx).await {
                let _ = chan.say(&ctx.http, format!("DM from {}:\n{}", &message.author, &message.content));
            };
        };
    }
}

//Generic handling of common user errors.
#[hook]
pub async fn dispatch_error(context: &Context, msg: &Message, error: DispatchError) {
    match error {
        DispatchError::NotEnoughArguments { min, given } => {
            let _ = msg
                .channel_id
                .say(
                    &context.http,
                    format!("Need {} arguments, but only got {}.", min, given),
                )
                .await;
        }
        DispatchError::TooManyArguments { max, given } => {
            let _ = msg
                .channel_id
                .say(
                    &context.http,
                    format!("Max arguments allowed is {}, but got {}.", max, given),
                )
                .await;
        }
        _ => error!("Unhandled dispatch error. {:?}", error),
    }
}

//Logs every command that errored, should only be used for bot failures and not user failures.
#[hook]
pub async fn after(_ctx: &Context, _msg: &Message, _cmd_name: &str, error: Result<(), CommandError>) {
    if let Err(why) = error {
        error!("{:?}", why);
    }
}

//Allows the use of a per-guild prefix with a default one set using the config file.
#[hook]
pub async fn dynamic_prefix(ctx: &Context, msg: &Message) -> Option<String> {
    let default_prefix: String;

    {
        let data = ctx.data.read();
        default_prefix = match data.await.get::<DefaultPrefix>() {
            Some(p) => p.clone(),
            None => return None,
        };
    }

    // Private messages use the default prefix.
    if msg.is_private() || msg.guild_id.is_none() {
        return Some(default_prefix.to_string());
    }

    let guildid = match msg.guild_id {
        Some(gid) => gid,
        None => return None,
    };

    // If the guild prefix is already retrieved, use it.
    {
        let data = ctx.data.read().await;
        let prefixes = match data.get::<GuildPrefixes>() {
            Some(p) => p,
            None => return None,
        };
        if let Some(x) = prefixes.get(&guildid) {
            return Some(x.to_string());
        }
    }

    // Otherwise, fetch it from the db.
    if let Ok(prefix) = get_prefix(guildid, &ctx).await {
        {
            let mut data = ctx.data.write().await;
            let prefixes = match data.get_mut::<GuildPrefixes>() {
                Some(p) => p,
                None => return None,
            };
            prefixes.insert(guildid, prefix.clone());
        }
        return Some(prefix);
    }

    // If there is no prefix set, use the default one.
    return Some(default_prefix);
}
