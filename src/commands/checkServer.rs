use crate::{Context, Error as PoiseError};
use async_minecraft_ping::{ConnectionConfig, StatusResponse};
use log::{error, info};
use poise::serenity_prelude::{self, model::connection, Colour};
use serde::{Deserialize, Serialize};

// Create a struct to represent the server argument.
#[derive(poise::ChoiceParameter)]
pub enum Server {
    POC3,
    Warpy,
    ProjectCreate,
}

#[poise::command(slash_command, prefix_command)]
pub async fn checkserver(ctx: Context<'_>, server: Server) -> Result<(), PoiseError> {
    let server_address = resolve_server_address(server);

    match check_server(server_address).await {
        Ok(status) => {
            send_server_status(ctx, server_address, status).await?;
            info!(
                "Server status for {} retrieved successfully",
                server_address
            );
        }
        /*
        Ok(None) => {
            ctx.say("Failed to get server status.").await?;
            error!("Failed to get server status for {}", server_address);
        }
        */
        Err(err) => {
            ctx.say(format!("Failed to get server status: {:?}", err))
                .await?;
            error!(
                "Error getting server status for {}: {:?}",
                server_address, err
            );
        }
    }

    Ok(())
}

fn resolve_server_address(server: Server) -> &'static str {
    match server {
        Server::POC3 => "poc3.openplayverse.net",
        Server::Warpy => "warpy.openplayverse.net",
        Server::ProjectCreate => "pc.openplayverse.net",
    }
}

async fn check_server(server: &str) -> anyhow::Result<async_minecraft_ping::StatusResponse> {
    let config = async_minecraft_ping::ConnectionConfig::build(server);
    let connection = config.connect().await?;
    let status = connection.status().await?.status;
    Ok(status)
}

async fn send_server_status(
    ctx: Context<'_>,
    server_address: &str,
    status: async_minecraft_ping::StatusResponse,
) -> Result<(), PoiseError> {
    let embed = serenity_prelude::CreateEmbed::default()
        .title(format!(
            "Server info for {}: {}",
            server_address,
            if status.version.name.is_empty() { "❌" } else { "✅" } 
        ))
        .color(Colour::from_rgb(0, 0, 255))
        .description(format!("```{:?}```", status.description))
        .field(
            "Total Players:",
            format!(
                "{}/{}",
                status.players.online.to_string(),
                status.players.max.to_string()
            ),
            true,
        )
        .field("Version:", status.version.name, true)
        .field("Protocol:", status.version.protocol.to_string(), true);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;

    Ok(())
}
