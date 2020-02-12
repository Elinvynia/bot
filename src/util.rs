use serenity::{
    framework::standard::{CommandError, DispatchError},
    model::prelude::*,
    prelude::*,
    utils::{parse_channel, parse_username},
};
use crate::data::{BotId, BotOwners, DefaultPrefix, GuildPrefixes};
use crate::db::get_prefix;
use log::error;

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

pub fn parse_user(
    name: &String,
    optional_gid: Option<&GuildId>,
    optional_ctx: Option<&Context>,
) -> Option<UserId> {
    if let Some(x) = parse_username(&name) {
        return Some(UserId(x));
    }

    if optional_gid.is_none() || optional_ctx.is_none() {
        return None;
    }

    let gid = optional_gid.unwrap();
    let ctx = optional_ctx.unwrap();

    let g = match gid.to_guild_cached(&ctx) {
        Some(g) => g,
        None => return None,
    };

    let guild = g.read();

    if let Ok(id) = name.parse::<u64>() {
        if let Ok(m) = guild.member(ctx, id) {
            return Some(m.user.read().id);
        }
    }

    if let Some(m) = guild.member_named(&name[..]) {
        return Some(m.user.read().id);
    }

    if let Some(m) = guild.members_starting_with(&name[..], false, true).get(0) {
        return Some(m.user.read().id);
    }

    if let Some(m) = guild.members_containing(&name[..], false, true).get(0) {
        return Some(m.user.read().id);
    }

    None
}

pub fn parse_chan(
    name: &String,
    optional_gid: Option<&GuildId>,
    optional_ctx: Option<&Context>,
) -> Option<ChannelId> {
    if let Some(x) = parse_channel(&name) {
        return Some(ChannelId(x));
    }

    if optional_gid.is_none() || optional_ctx.is_none() {
        return None;
    }

    let gid = optional_gid.unwrap();
    let ctx = optional_ctx.unwrap();

    if let Ok(id) = name.parse::<u64>() {
        if let Some(x) = ChannelId(id).to_channel_cached(&ctx) {
            return Some(x.id());
        }
    }

    let g = match gid.to_guild_cached(&ctx) {
        Some(g) => g,
        None => return None,
    };

    let guild = g.read();

    for (key, value) in guild.channels.iter() {
        let cname = &value.read().name;
        if cname == name {
            return Some(*key);
        }
        if cname.contains(name) {
            return Some(*key);
        }
    }

    None
}

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

pub fn after(_ctx: &mut Context, _msg: &Message, _cmd_name: &str, error: Result<(), CommandError>) {
    if let Err(why) = error {
        error!("{:?}", why);
    }
}

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
            .map_or_else(|_| default_prefix, |pref| pref).to_string(),
    );
}
