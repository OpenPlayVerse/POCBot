use crate::{Context, Error as PoiseError};
use log::{error, info};
use poise::serenity_prelude::{self, Colour};
use crate::utils::check::{get_server_status_from_api, McSrvStatResponse};

#[derive(poise::ChoiceParameter)]
pub enum Server {
    POC3,
    Warpy,
    ProjectCreate,
}

#[poise::command(slash_command, prefix_command)]
pub async fn checkserver(ctx: Context<'_>, server: Server) -> Result<(), PoiseError> {
    let server_address = resolve_server_address(server);
    
    match get_server_status_from_api(server_address).await {
        Ok(status) => {
            if status.online {
                send_server_status_from_api(ctx, server_address, status).await?;
                info!(
                    "Server status for {} retrieved successfully via API",
                    server_address
                );
            } else {
                ctx.say(format!("Server {} is offline according to the API.", server_address)).await?;
                info!("Server {} is offline via API", server_address);
            }
        }
        Err(e) => {
            ctx.say(format!("Error fetching server status for {}: {}", server_address, e)).await?;
            error!("Error getting server status for {} via API: {}", server_address, e);
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

async fn send_server_status_from_api(
    ctx: Context<'_>,
    server_address: &str,
    status: McSrvStatResponse,
) -> Result<(), PoiseError> {
    let title = format!(
        "Server info for {}: {}",
        status.hostname.as_deref().unwrap_or(server_address),
        if status.online { "✅" } else { "❌" }
    );
    let color = if status.online { Colour::DARK_GREEN } else { Colour::RED };

    let mut embed = serenity_prelude::CreateEmbed::default()
        .title(title)
        .color(color);

    embed = embed.description(status.motd.as_ref().map_or_else(|| "No MOTD available.".to_string(), |m| m.clean.join("\n")));

    if let Some(players) = &status.players {
        let player_count_text = format!("{}/{}", players.online.unwrap_or(0), players.max.unwrap_or(0));
        embed = embed.field("Players:", player_count_text, true);

        if let Some(list) = &players.list {
            if !list.is_empty() {
                let player_list_text = list.iter().map(|p| p.name.as_str()).collect::<Vec<&str>>().join(", ");
                embed = embed.field("Player List:", player_list_text, true);
            }
        }
    } else {
        embed = embed.field("Players:", "N/A", true);
    }
    
    embed = embed.field("Version:", status.version.as_deref().unwrap_or("N/A"), true);

    if let Some(protocol) = &status.protocol {
        embed = embed.field("Protocol:", protocol.name.as_deref().unwrap_or("N/A"), true);
    }
    
    // Use the direct icon endpoint from mcsrvstat.us
    let icon_url = format!("https://api.mcsrvstat.us/icon/{}", server_address);
    embed = embed.thumbnail(icon_url);


    ctx.send(poise::CreateReply::default().embed(embed)).await?;

    Ok(())
}
