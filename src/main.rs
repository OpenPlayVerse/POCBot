mod commands;

use dotenv::dotenv;
use poise::serenity_prelude as serenity;

/// A shared instance of this struct is available across all events and framework commands
pub struct Data {
    command_counter: std::sync::Mutex<std::collections::HashMap<String, u64>>,
}
/// This Error type is used throughout all commands and callbacks
type Error = Box<dyn std::error::Error + Send + Sync>;

/// This type alias will save us some typing, because the Context type is needed often
type Context<'a> = poise::Context<'a, Data, Error>;

async fn event_event_handler(
    _ctx: &serenity::Context,
    event: &poise::Event<'_>,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _user_data: &Data,
) -> Result<(), Error> {
    match event {
        poise::Event::Ready { data_about_bot } => {
            println!("{} is connected!", data_about_bot.user.name)
        }
        _ => {}
    }

    Ok(())
}

async fn pre_command(ctx: Context<'_>) {
    println!(
        "Got command '{}' by user '{}'",
        ctx.command().name,
        ctx.author().name
    );

    // Increment the number of times this command has been run once. If
    // the command's name does not exist in the counter, add a default
    // value of 0.
    let mut command_counter = ctx.data().command_counter.lock().unwrap();
    let entry = command_counter
        .entry(ctx.command().name.to_string())
        .or_insert(0);
    *entry += 1;
}

async fn post_command(ctx: Context<'_>) {
    println!("Processed command '{}'", ctx.command().name);
}

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::Command { error, ctx } => {
            println!(
                "Command '{}' returned error {:?}",
                ctx.command().name,
                error
            );
        }
        poise::FrameworkError::EventHandler { error, event, .. } => {
            println!(
                "EventHandler returned error during {:?} event: {:?}",
                event.name(),
                error
            );
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", e)
            }
        }
    }
}


#[tokio::main]
async fn main() {
	dotenv().ok();

    let options = poise::FrameworkOptions {
        commands: vec![
            // This function registers slash commands on Discord. When you change something about a
            // command signature, for example by changing its name, adding or removing parameters, or
            // changing a parameter type, you should call this function.
            commands::register::register(),
			commands::checkServer::checkserver(),
			commands::ping::ping()
        ],
        event_handler: |ctx, event, framework, user_data| {
            Box::pin(event_event_handler(ctx, event, framework, user_data))
        },
        on_error: |error| Box::pin(on_error(error)),
        // Set a function to be called prior to each command execution. This
        // provides all context of the command that would also be passed to the actual command code
        pre_command: |ctx| Box::pin(pre_command(ctx)),
        // Similar to `pre_command`, except will be called directly _after_
        // command execution.
        post_command: |ctx| Box::pin(post_command(ctx)),

        // Options specific to prefix commands, i.e. commands invoked via chat messages
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some(String::from("~")),

            mention_as_prefix: false,
            // An edit tracker needs to be supplied here to make edit tracking in commands work
            edit_tracker: Some(poise::EditTracker::for_timespan(
                std::time::Duration::from_secs(3600 * 3),
            )),
            ..Default::default()
        },

        ..Default::default()
    };

    // The Framework builder will automatically retrieve the bot owner and application ID via the
    // passed token, so that information need not be passed here
    poise::Framework::builder()
        // Configure the client with your Discord bot token in the environment.
        .token(std::env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN in the environment"))
        .options(options)
		.intents(serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT)
        .setup(|_ctx, _data_about_bot, _framework| {
            Box::pin(async move {
                Ok(Data {
                    command_counter: std::sync::Mutex::new(std::collections::HashMap::new()),
                })
            })
        })
        .run()
        .await
        .expect("Client error");
}