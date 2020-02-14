use serenity::{
    model::prelude::*,
    prelude::*,
    utils::{parse_channel, parse_username},
};

// A more detailed user parsing function
// Priority of parsing:
// 1. Mention
// 2. User ID
// 3. User name
// 4. User name starting with
// 5. User name containing
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

// A more detailed channel parsing function
// Priority of parsing:
// 1. Mention
// 2. Channel ID
// 3. Channel name
// 4. Part of a channel name
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
    }

    for (key, value) in guild.channels.iter() {
        let cname = &value.read().name;
        if cname.contains(name) {
            return Some(*key);
        }
    }

    None
}
