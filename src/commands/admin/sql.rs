use crate::prelude::*;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};
use sqlx::prelude::*;

#[command]
#[owners_only]
#[num_args(1)]
#[description("SQL query.")]
#[usage("sql <query>")]
#[example("sql \"DROP TABLE students\"")]
async fn sql(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut conn = connect().await?;
    let query: String = error_return_ok!(args.single_quoted());

    let mut message = String::new();
    let mut row_counter = 0_u64;
    let mut rows = sqlx::query(&query).fetch(&mut conn);

    while let Ok(Some(row)) = rows.next().await {
        message += &format!("Row {}:\n", row_counter);
        for x in 0..row.len() {
            message += row.get_unchecked(x);
            message += "\n";
        }
        message += "\n";
        row_counter += 1;
    }

    msg.channel_id.say(&ctx, &format!("Result:\n{}", message)).await?;

    Ok(())
}
