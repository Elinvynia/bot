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
pub async fn parse_user(
    name: &str,
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

    let guild = match gid.to_guild_cached(&ctx).await {
        Some(g) => g,
        None => return None,
    };

    if let Ok(id) = name.parse::<u64>() {
        if let Ok(m) = guild.member(ctx, id).await {
            return Some(m.user.id);
        }
    }

    if let Some(m) = guild.member_named(&name[..]).await {
        return Some(m.user.id);
    }

    if let Some(m) = guild
        .members_starting_with(&name[..], false, true)
        .await
        .get(0)
    {
        let (mem, _) = m;
        return Some(mem.user.id);
    }

    if let Some(m) = guild
        .members_containing(&name[..], false, true)
        .await
        .get(0)
    {
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

    if optional_gid.is_none() || optional_ctx.is_none() {
        return None;
    }

    let gid = optional_gid.unwrap();
    let ctx = optional_ctx.unwrap();

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
