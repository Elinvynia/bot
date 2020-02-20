use serenity::{
    client::Context,
    framework::standard::{macros::help, Args, CommandGroup, CommandResult, HelpOptions},
    model::prelude::*,
};
use std::collections::HashSet;

#[help]
fn help(
    ctx: &mut Context,
    msg: &Message,
    _args: Args,
    _help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    _owners: HashSet<UserId>,
) -> CommandResult {
    let mut s = "Eli Bot made by @Elinvynia".to_string();
    s.push_str("\n\n");
    for x in groups {
        let mut n = format!("**{}:** ", x.name);
        for y in x.options.commands {
            let name = y.options.names.first().unwrap();
            n.push_str(&format!("{}, ", &name)[..]);
        }
        n.push_str("\n");
        s.push_str(&n[..])
    }
    msg.channel_id.say(&ctx, &s)?;

    Ok(())
}
