use crate::data::error::BotError;
use serenity::{
    client::Context,
    framework::standard::{macros::help, Args, Command, CommandGroup, CommandResult, HelpOptions, OnlyIn},
    model::prelude::*,
};
use std::collections::HashSet;

#[help]
async fn help(
    ctx: &Context,
    msg: &Message,
    args: Args,
    _help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let mut help_string = String::new();

    match args.len() {
        0 => help_string += &command_list(ctx, groups, msg, owners).await?,
        1 => {
            help_string += &command_help(
                ctx,
                groups,
                args.current().ok_or(BotError::NoneError)?.to_string(),
                msg,
                owners,
            ).await?
        }
        _ => help_string += "Too many arguments.",
    };

    msg.channel_id.say(&ctx, &help_string).await?;

    Ok(())
}

async fn command_list(ctx: &Context, groups: &[&'static CommandGroup], msg: &Message, owners: HashSet<UserId>) -> Result<String, BotError> {
    let mut help_list = "Bot made by @Elinvynia".to_string();
    help_list += "\n\n";

    let is_owner = owners.get(&msg.author.id).is_some();

    for group in groups {
        let mut group_string = format!("**{}:** ", group.name);
        for command in group.options.commands {
            let name = command.options.names.first().ok_or(BotError::NoneError)?;
            let mut got_permission = false;

            if let Some(gid) = msg.guild_id {
                let guild = match gid.to_guild_cached(&ctx).await {
                    Some(g) => g,
                    None => return Err(BotError::NoneError),
                };
                let member = match guild.member(&ctx, msg.author.id).await {
                    Ok(m) => m,
                    Err(_) => return Err(BotError::NoneError),
                };
                let rid = match member.highest_role_info(&ctx).await {
                    Some(id) => id,
                    None => return Err(BotError::NoneError),
                };
                let role = match rid.0.to_role_cached(&ctx).await {
                    Some(r) => r,
                    None => return Err(BotError::NoneError),
                };
                got_permission = role.permissions.contains(command.options.required_permissions);
            };

            if command.options.owners_only && !is_owner {
                group_string += &format!("~~{}~~, ", &name);
            } else if command.options.only_in == OnlyIn::Guild && msg.guild_id.is_none() {
                group_string += &format!("~~{}~~, ", &name);
            } else if !got_permission && msg.guild_id.is_some() {
                group_string += &format!("~~{}~~, ", &name);
            } else {
                group_string += &format!("{}, ", &name);
            }
        }
        group_string += "\n";
        help_list += &group_string;
        help_list.remove(help_list.len() - 3);
    }
    help_list += "\n";
    help_list += "source @ <https://github.com/Elinvynia/bot>";
    Ok(help_list)
}

async fn command_help(
    ctx: &Context,
    groups: &[&'static CommandGroup],
    arg: String,
    msg: &Message,
    owners: HashSet<UserId>,
) -> Result<String, BotError> {
    let mut help_command = String::new();

    let is_owner = owners.get(&msg.author.id).is_some();

    let mut matched_command: Option<&Command> = None;
    for group in groups {
        for command in group.options.commands {
            let name = command.options.names.first().ok_or(BotError::NoneError)?;
            if name == &arg {
                matched_command = Some(command);
            }
        }
    }

    let command = match matched_command {
        Some(c) => c,
        None => {
            help_command += "No command found.";
            return Ok(help_command);
        }
    };

    let mut got_permission = false;

    if let Some(gid) = msg.guild_id {
        let guild = match gid.to_guild_cached(&ctx).await {
            Some(g) => g,
            None => return Ok(help_command),
        };
        let member = match guild.member(&ctx, msg.author.id).await {
            Ok(m) => m,
            Err(_) => return Ok(help_command),
        };
        let rid = match member.highest_role_info(&ctx).await {
            Some(id) => id,
            None => return Ok(help_command),
        };
        let role = match rid.0.to_role_cached(&ctx).await {
            Some(r) => r,
            None => return Ok(help_command),
        };
        got_permission = role.permissions.contains(command.options.required_permissions);
    };

    help_command += &format!(
        "**Command:** __{}__",
        command.options.names.first().ok_or(BotError::NoneError)?
    );

    help_command += "\n";
    if let Some(description) = command.options.desc {
        help_command += &format!("**Description:** {}", description);
    } else {
        help_command += "No description available.";
    }

    help_command += "\n";
    if let Some(usage) = command.options.usage {
        help_command += &format!("**Usage:** {}", usage);
    } else {
        help_command += "No usage available.";
    }

    help_command += "\n";
    if !command.options.examples.is_empty() {
        help_command += "**Examples:** ";
        for example in command.options.examples {
            help_command += example;
        }
    } else {
        help_command += "No examples available.";
    }

    help_command += "\n";
    if command.options.owners_only && !is_owner {
        help_command += "You need to be an owner to use this command.";
    } else if command.options.only_in == OnlyIn::Guild {
        if msg.guild_id.is_none() {
            help_command += "This command can only be used in a guild.";
        } else if !got_permission {
            help_command += "You don't have the permissions to use this command.";
        }
    };

    Ok(help_command)
}
