use serenity::{
    model::prelude::*,
    prelude::*,
    utils::{parse_channel, parse_emoji, parse_role, parse_username},
};

// A more detailed user parsing function
// Priority of parsing:
// 1. Mention
// 2. User ID
// 3. User name
// 4. User name starting with
// 5. User name containing
pub async fn parse_user(name: &str, optional_gid: Option<&GuildId>, optional_ctx: Option<&Context>) -> Option<UserId> {
    if let Some(x) = parse_username(&name) {
        return Some(UserId(x));
    }

    let gid = match optional_gid {
        Some(g) => g,
        None => return None,
    };

    let ctx = match optional_ctx {
        Some(c) => c,
        None => return None,
    };

    let guild = match gid.to_guild_cached(&ctx).await {
        Some(g) => g,
        None => return None,
    };

    if let Ok(id) = name.parse::<u64>() {
        if let Ok(m) = guild.member(ctx, id).await {
            return Some(m.user.id);
        }
    }

    if let Some(m) = guild.member_named(&name[..]) {
        return Some(m.user.id);
    }

    if let Some(m) = guild.members_starting_with(&name[..], false, true).await.get(0) {
        let (mem, _) = m;
        return Some(mem.user.id);
    }

    if let Some(m) = guild.members_containing(&name[..], false, true).await.get(0) {
        let (mem, _) = m;
        return Some(mem.user.id);
    }

    None
}

// A more detailed channel parsing function
// Priority of parsing:
// 1. Mention
// 2. Channel ID
// 3. Channel name
// 4. Part of a channel name
pub async fn parse_chan(
    name: &str,
    optional_gid: Option<&GuildId>,
    optional_ctx: Option<&Context>,
) -> Option<ChannelId> {
    if let Some(x) = parse_channel(&name) {
        return Some(ChannelId(x));
    }

    let gid = match optional_gid {
        Some(g) => g,
        None => return None,
    };

    let ctx = match optional_ctx {
        Some(c) => c,
        None => return None,
    };

    if let Ok(id) = name.parse::<u64>() {
        if let Some(x) = ChannelId(id).to_channel_cached(&ctx).await {
            return Some(x.id());
        }
    }

    let guild = match gid.to_guild_cached(&ctx).await {
        Some(g) => g,
        None => return None,
    };

    for (key, value) in guild.channels.iter() {
        let cname = &value.name;
        if cname == name {
            return Some(*key);
        }
    }

    for (key, value) in guild.channels.iter() {
        let cname = &value.name;
        if cname.contains(name) {
            return Some(*key);
        }
    }

    None
}

// A more detailed role parsing function
// Priority of parsing:
// 1. Mention
// 2. Role ID
// 3. Role name
// 4. Part of a role name
pub async fn parse_rol(name: &str, optional_gid: Option<&GuildId>, optional_ctx: Option<&Context>) -> Option<RoleId> {
    if let Some(x) = parse_role(&name) {
        return Some(RoleId(x));
    }

    if optional_gid.is_none() || optional_ctx.is_none() {
        return None;
    }

    let gid = match optional_gid {
        Some(g) => g,
        None => return None,
    };

    let ctx = match optional_ctx {
        Some(c) => c,
        None => return None,
    };

    if let Ok(id) = name.parse::<u64>() {
        if let Some(x) = RoleId(id).to_role_cached(&ctx).await {
            return Some(x.id);
        }
    }

    let guild = match gid.to_guild_cached(&ctx).await {
        Some(g) => g,
        None => return None,
    };

    for (key, value) in guild.roles.iter() {
        let rname = &value.name;
        if rname == name {
            return Some(*key);
        }
    }

    for (key, value) in guild.roles.iter() {
        let rname = &value.name;
        if rname.contains(name) {
            return Some(*key);
        }
    }

    None
}

// A more detailed reaction parsing function
// Priority of parsing:
// 1. Reaction
// 2. Reaction ID
// 3. Reaction name
// 4. Part of a reaction name
pub async fn parse_reaction(name: &str, gid: &GuildId, ctx: &Context) -> Option<Emoji> {
    let guild = match gid.to_guild_cached(&ctx).await {
        Some(g) => g,
        None => return None,
    };

    if let Some(x) = parse_emoji(name) {
        for (_key, value) in guild.emojis.iter() {
            let ename = &value.name;
            if ename == &x.name {
                return Some(value.clone());
            }
        }
    }

    if let Ok(id) = name.parse::<u64>() {
        for (_key, value) in guild.emojis.iter() {
            let eid = value.id.as_u64();
            if eid == &id {
                return Some(value.clone());
            }
        }
    }

    for (_key, value) in guild.emojis.iter() {
        let ename = &value.name;
        if ename == name {
            return Some(value.clone());
        }
    }

    for (_key, value) in guild.emojis.iter() {
        let ename = &value.name;
        if ename.contains(name) {
            return Some(value.clone());
        }
    }

    None
}
