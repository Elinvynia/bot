use evcxr::EvalContext;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};

#[command]
#[owners_only]
#[description("Runs Rust code without any packages. Use with caution.")]
#[usage("exec code | ```code```")]
#[example("exec \"print!(\"hello!\")\"")]
async fn exec(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let arg = args.message().replace("```", "");

    let (mut context, outputs) = EvalContext::new()?;
    let mut state = context.state();
    state.set_opt_level("0")?;
    let mut output = match context.eval_with_state(&arg, state) {
        Ok(_) => String::new(),
        Err(e) => e.to_string(),
    };

    if output.is_empty() {
        let mut stdouts = vec![];
        while let Ok(s) = outputs.stdout.try_recv() {
            stdouts.push(s)
        }
        output = stdouts.join("\n");
    }

    msg.channel_id.say(&ctx, &format!("Output:\n{}", output)).await?;

    Ok(())
}
