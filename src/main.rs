mod commands;

use dotenv::dotenv;
use poise::serenity_prelude as serenity;
use tokio::sync::Mutex;
use std::collections::HashMap;
use log::{info, error};

/// A shared instance of this struct is available across all events and framework commands
pub struct Data {
    command_counter: Mutex<HashMap<String, u64>>,
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
            info!("{} is connected!", data_about_bot.user.name);
        }
        _ => {}
    }

    Ok(())
}

async fn pre_command(ctx: Context<'_>) {
    info!(
        "Got command '{}' by user '{}'",
        ctx.command().name,
        ctx.author().name
    );

    let mut command_counter = ctx.data().command_counter.lock().await;
    let entry = command_counter
        .entry(ctx.command().name.to_string())
        .or_insert(0);
    *entry += 1;
}

async fn post_command(ctx: Context<'_>) {
    info!("Processed command '{}'", ctx.command().name);
}

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::Command { error, ctx } => {
            error!(
                "Command '{}' returned error {:?}",
                ctx.command().name,
                error
            );
        }
        poise::FrameworkError::EventHandler { error, event, .. } => {
            error!(
                "EventHandler returned error during {:?} event: {:?}",
                event.name(),
                error
            );
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                error!("Error while handling error: {}", e);
            }
        }
    }
}

fn register_commands() -> Vec<poise::Command<Data, Error>> {
    vec![
        commands::register::register(),
        commands::checkServer::checkserver(),
        commands::ping::ping(),
        commands::news::news(),
    ]
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let token = std::env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN in the environment");

    let options = poise::FrameworkOptions {
        commands: register_commands(),
        event_handler: |ctx, event, framework, user_data| {
            Box::pin(event_event_handler(ctx, event, framework, user_data))
        },
        on_error: |error| Box::pin(on_error(error)),
        pre_command: |ctx| Box::pin(pre_command(ctx)),
        post_command: |ctx| Box::pin(post_command(ctx)),
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some(String::from("~")),
            mention_as_prefix: false,
            edit_tracker: Some(poise::EditTracker::for_timespan(
                std::time::Duration::from_secs(3600 * 3),
            )),
            ..Default::default()
        },
        ..Default::default()
    };

    poise::Framework::builder()
        .token(token)
        .options(options)
        .intents(serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT)
        .setup(|_ctx, _data_about_bot, _framework| {
            Box::pin(async move {
                Ok(Data {
                    command_counter: Mutex::new(HashMap::new()),
                })
            })
        })
        .run()
        .await
        .expect("Client error");
}