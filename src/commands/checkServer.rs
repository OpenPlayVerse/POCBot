use crate::{Context, Error as PoiseError};
use log::{error, info};
use poise::serenity_prelude::{self, Colour};
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
        Ok(Some(status)) => {
            send_server_status(ctx, server_address, status).await?;
            info!(
                "Server status for {} retrieved successfully",
                server_address
            );
        }
        Ok(None) => {
            ctx.say("Failed to get server status.").await?;
            error!("Failed to get server status for {}", server_address);
        }
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

async fn check_server(server: &str) -> anyhow::Result<Option<ServerData>> {
    let url = format!("https://api.mcsrvstat.us/3/{}", server);
    let response = reqwest::get(&url).await?.json::<ServerData>().await?;

    Ok(Some(response))
}

async fn send_server_status(
    ctx: Context<'_>,
    server_address: &str,
    status: ServerData,
) -> Result<(), PoiseError> {
    let mut embed = serenity_prelude::CreateEmbed::default()
        .title(format!(
            "Server info for {}: {}",
            server_address,
            if status.online { "✅" } else { "❌" }
        ))
        .color(Colour::from_rgb(0, 0, 255));

    if let Some(motd) = status.motd.as_ref() {
        embed = embed.description(format!("```{}```", motd.raw.join("\n")));
    }

    if let Some(players) = status.players.as_ref() {
        embed = embed.field(
            "Total Players:",
            format!("{}/{}", players.online, players.max),
            true,
        );
    }

    if let Some(version) = status.version.as_ref() {
        embed = embed.field("Version:", version, true);
    }

    if let Some(software) = status.software.as_ref() {
        embed = embed.field("Software:", software, true);
    }

    ctx.send(poise::CreateReply::default().embed(embed)).await?;

    Ok(())
}

#[derive(Serialize, Deserialize)]
pub struct ServerData {
    online: bool,
    ip: Option<String>,
    port: Option<i64>,
    hostname: Option<String>,
    debug: Debug,
    version: Option<String>,
    protocol: Option<Protocol>,
    icon: Option<String>,
    software: Option<String>,
    map: Option<Map>,
    gamemode: Option<String>,
    serverid: Option<String>,
    eula_blocked: Option<bool>,
    motd: Option<Motd>,
    players: Option<Players>,
    plugins: Option<Vec<Plugin>>,
    mods: Option<Vec<Mod>>,
    info: Option<Info>,
}

#[derive(Serialize, Deserialize)]
pub struct Debug {
    ping: bool,
    query: bool,
    srv: bool,
    querymismatch: bool,
    ipinsrv: bool,
    cnameinsrv: bool,
    animatedmotd: bool,
    cachehit: bool,
    cachetime: i64,
    cacheexpire: i64,
    apiversion: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Protocol {
    version: i64,
    name: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Map {
    raw: String,
    clean: String,
    html: String,
}

#[derive(Serialize, Deserialize)]
pub struct Motd {
    raw: Vec<String>,
    clean: Vec<String>,
    html: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Players {
    online: i64,
    max: i64,
    list: Option<Vec<Player>>,
}

#[derive(Serialize, Deserialize)]
pub struct Player {
    name: String,
    uuid: String,
}

#[derive(Serialize, Deserialize)]
pub struct Plugin {
    name: String,
    version: String,
}

#[derive(Serialize, Deserialize)]
pub struct Mod {
    name: String,
    version: String,
}

#[derive(Serialize, Deserialize)]
pub struct Info {
    raw: Vec<String>,
    clean: Vec<String>,
    html: Vec<String>,
}
