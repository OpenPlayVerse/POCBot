mod commands;

use dotenv::dotenv;
use log::{error, info};
use poise::serenity_prelude as serenity;
use std::collections::HashMap;
use tokio::sync::Mutex;
use tokio::time::Duration;

/// A shared instance of this struct is available across all events and framework commands
pub struct Data {
    command_counter: Mutex<HashMap<String, u64>>,
}

/// This Error type is used throughout all commands and callbacks
type Error = Box<dyn std::error::Error + Send + Sync>;

/// This type alias will save us some typing, because the Context type is needed often
type Context<'a> = poise::Context<'a, Data, Error>;

async fn event_event_handler<'a>(
    _ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'a, Data, Error>,
    _user_data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot } => {
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
        poise::FrameworkError::Command { error, ctx, .. } => {
            error!(
                "Command '{}' returned error {:?}",
                ctx.command().name,
                error
            );
        }
        poise::FrameworkError::EventHandler { error, event, .. } => {
            error!(
                "EventHandler returned error during {:?} event: {:?}",
                event.snake_case_name(),
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

    let token = std::env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN in the environment");
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;
    let options = poise::FrameworkOptions {
        commands: register_commands(),
        event_handler: |ctx, event, framework, user_data| {
            Box::pin(event_event_handler(ctx, event, framework, user_data))
        },
        on_error: |error| Box::pin(on_error(error)),
        pre_command: |ctx| Box::pin(pre_command(ctx)),
        post_command: |ctx| Box::pin(post_command(ctx)),
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("~".into()),
            mention_as_prefix: false,
            edit_tracker: Some(
                poise::EditTracker::for_timespan(Duration::from_secs(3600 * 3)).into(),
            ),
            ..Default::default()
        }, 
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .options(options)
        .setup(|_ctx, _ready, _framework| {
            Box::pin(async move {
                Ok(Data {
                    command_counter: Mutex::new(HashMap::new()),
                })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap()
}
