use crate::util::parse_user;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};

#[command]
#[min_args(0)]
#[max_args(1)]
fn user(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
	let userid = match parse_user(&args.quoted().current().unwrap().to_string(), msg.guild_id.as_ref(), Some(&ctx)) {
		Some(id) => id,
		None => return Ok(()),
	};
	let user = userid.to_user(&ctx)?;

	let _ = msg.channel_id.say(&ctx, format!("User found!\nTag: {}\nID: {}", user.tag(), user.id));
    Ok(())
}
