use crate::prelude::*;
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::*,
    prelude::*,
};

#[command]
#[only_in(guilds)]
#[description("Retrieves current server information.")]
#[usage("guild")]
#[example("guild")]
async fn guild(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg
        .guild_id
        .ok_or(BotError::NoneError)?
        .to_guild_cached(&ctx)
        .await
        .ok_or(BotError::NoneError)?;

    let owner = guild
        .owner_id
        .to_user(ctx)
        .await
        .map_or("Owner information not found.".to_string(), |user| user.tag());

    let filter = match guild.explicit_content_filter {
        ExplicitContentFilter::None => "Don't scan any messages.",
        ExplicitContentFilter::WithoutRole => "Scan messages from members without a role.",
        ExplicitContentFilter::All => "Scan messages sent by all members.",
        _ => "ERROR",
    };

    let notifications = match guild.default_message_notifications {
        DefaultMessageNotificationLevel::All => "Receive notifications for everything.",
        DefaultMessageNotificationLevel::Mentions => "Receive only mentions.",
        _ => "ERROR",
    };

    let verification = match guild.verification_level {
        VerificationLevel::None => "Does not require any verification.",
        VerificationLevel::Low => "Must have a verified email on the user's Discord account.",
        VerificationLevel::Medium => "Must also be a registered user on Discord for longer than 5 minutes.",
        VerificationLevel::High => "Must also be a member of the guild for longer than 10 minutes.",
        VerificationLevel::Higher => "Must have a verified phone on the user's Discord account.",
        _ => "ERROR",
    };

    let tier = match guild.premium_tier {
        PremiumTier::Tier0 => "Tier 0",
        PremiumTier::Tier1 => "Tier 1",
        PremiumTier::Tier2 => "Tier 2",
        PremiumTier::Tier3 => "Tier 3",
        _ => "ERROR",
    };

    let mut message = format!("__**{}**__\n\n", guild.name);
    message += &format!("**ID:** {}\n", guild.id);
    message += &format!(
        "**Description:** {}\n",
        guild.description.unwrap_or_else(|| "None.".into())
    );
    message += &format!("**Members:** {}\n", guild.member_count);
    message += &format!("**Created At:** {}\n", guild.member_count);
    message += &format!("**Large:** {}\n", guild.large);
    message += &format!("**Premium Tier:** {}\n", tier);
    message += &format!("**Boosters:** {}\n", guild.premium_subscription_count);
    message += &format!("**Owner:** {}\n", owner);
    message += &format!("**Region:** {}\n", guild.region);
    message += &format!("**Content Filter:** {}\n", filter);
    message += &format!("**Notification Level:** {}\n", notifications);
    message += &format!("**Verification Level:** {}\n", verification);

    msg.channel_id.say(&ctx.http, message).await?;

    Ok(())
}
