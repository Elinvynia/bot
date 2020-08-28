use crate::data::error::BotError;
use serenity::{
    client::Context,
    framework::standard::{macros::help, Args, Command, CommandGroup, CommandResult, HelpOptions},
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
    _owners: HashSet<UserId>,
) -> CommandResult {
    let mut s = String::new();

    match args.len() {
        0 => s.push_str(&command_list(groups)?),
        1 => s.push_str(&command_help(
            groups,
            args.current().ok_or(BotError::NoneError)?.to_string(),
        )?),
        _ => s.push_str("Too many arguments."),
    };

    msg.channel_id.say(&ctx, &s).await?;

    Ok(())
}

fn command_list(groups: &[&'static CommandGroup]) -> Result<String, BotError> {
    let mut s = "Bot made by @Elinvynia".to_string();
    s.push_str("\n\n");
    for x in groups {
        let mut n = format!("**{}:** ", x.name);
        for y in x.options.commands {
            let name = y.options.names.first().ok_or(BotError::NoneError)?;
            n.push_str(&format!("{}, ", &name)[..]);
        }
        n.push_str("\n");
        s.push_str(&n[..]);
        s.remove(s.len() - 3);
    }
    s.push_str("\n");
    s.push_str("source @ <https://github.com/Elinvynia/bot>");
    Ok(s)
}

fn command_help(groups: &[&'static CommandGroup], arg: String) -> Result<String, BotError> {
    let mut s = String::new();

    let mut matched_command: Option<&Command> = None;
    for x in groups {
        for y in x.options.commands {
            let name = y.options.names.first().ok_or(BotError::NoneError)?;
            if name == &arg {
                matched_command = Some(y);
            }
        }
    }

    let command = match matched_command {
        Some(c) => c,
        None => {
            s.push_str("No command found.");
            return Ok(s);
        }
    };

    s.push_str(&format!(
        "**Command:** __{}__",
        command.options.names.first().ok_or(BotError::NoneError)?
    ));

    s.push_str("\n");
    if let Some(description) = command.options.desc {
        s.push_str("**Description:** ");
        s.push_str(description);
    } else {
        s.push_str("No description available.");
    }

    s.push_str("\n");
    if let Some(usage) = command.options.usage {
        s.push_str("**Usage:** ");
        s.push_str(usage);
    } else {
        s.push_str("No usage available.");
    }

    s.push_str("\n");
    if !command.options.examples.is_empty() {
        s.push_str("**Examples:** ");
        for x in command.options.examples {
            s.push_str(x);
        }
    } else {
        s.push_str("No examples available.")
    }
    Ok(s)
}
