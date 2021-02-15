use crate::prelude::*;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};

macro_rules! text_command {
    ($cname:ident, $action:expr, $desc:expr, $usage:expr, $example:expr, $extra:expr) => {
        #[command]
        #[only_in(guilds)]
        #[num_args(1)]
        #[description($desc)]
        #[usage($usage)]
        #[example($example)]
        async fn $cname(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
            let gid = msg.guild_id.ok_or_else(|| anyhow!("Guild ID not found."))?;

            let user_id = none_return_ok!(parse_user(&args.single::<String>()?, Some(&gid), Some(&ctx)).await);
            let author = gid.member(&ctx, msg.author.id).await?;
            let member = gid.member(&ctx, user_id).await?;

            msg.channel_id
                .say(
                    &ctx,
                    format!(
                        "***{}** {} **{}**{}*",
                        author.display_name(),
                        $action,
                        member.display_name(),
                        $extra
                    ),
                )
                .await?;

            Ok(())
        }
    };
}

text_command!(hug, "hugs", "Hugs another user.", "hug <user>", "hug Elinvynia", "");
text_command!(
    headpat,
    "headpats",
    "Headpats another user.",
    "headpat <user>",
    "headpat Elinvynia",
    ""
);
text_command!(
    bonk,
    "bonks",
    "Bonks another user to horny jail.",
    "bonk <user>",
    "bonk Elinvynia",
    ". Go to horny jail!"
);
