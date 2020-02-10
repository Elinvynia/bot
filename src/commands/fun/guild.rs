use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::*,
    prelude::*,
};

#[command]
#[only_in(guilds)]
fn guild(ctx: &mut Context, msg: &Message) -> CommandResult {
    let g = match msg.guild_id.unwrap().to_guild_cached(&ctx) {
        Some(g) => g,
        None => {
            let _ = msg.channel_id.say(&ctx.http, "Guild not found in cache.");
            return Ok(());
        }
    };

    let guild = g.read();
    let owner = guild
        .owner_id
        .to_user(&ctx)
        .map_or("Owner information not found".to_string(), |user| {
            format!("<@{}>", user.id.to_string())
        });

    let _ = msg.channel_id.say(
        &ctx.http,
        format!(
            "Guild {}\nOwner: {}\n Region: {}\n",
            guild.name, owner, guild.region
        ),
    );

    Ok(())
}
