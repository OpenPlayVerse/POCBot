use crate::{Context, Error as PoiseError};
use poise::serenity_prelude::Colour;
use serde::{Deserialize, Serialize};
use log::{info, error};

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
            info!("Server status for {} retrieved successfully", server_address);
        }
        Ok(None) => {
            ctx.say("Failed to get server status.").await?;
            error!("Failed to get server status for {}", server_address);
        }
        Err(err) => {
            ctx.say(format!("Failed to get server status. {:?}", err)).await?;
            error!("Error getting server status for {}: {:?}", server_address, err);
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

async fn send_server_status(ctx: Context<'_>, server_address: &str, status: ServerData) -> Result<(), PoiseError> {
    poise::send_reply(ctx, |m| {
        m.embed(|e| {
            e.title(format!(
                "Server info for {}: {}",
                server_address,
                if status.online { "✅" } else { "❌" }
            ));

            if let Some(motd) = status.motd.as_ref() {
                e.description(format!("```{}```", motd.raw.join("\n")));
            }

            e.color(Colour::from_rgb(0, 0, 255));

            if let Some(players) = status.players.as_ref() {
                e.field(
                    "Total Players:",
                    format!("{}/{}", players.online, players.max),
                    true,
                );
            }

            e
        })
    })
    .await?;

    Ok(())
}

#[derive(Serialize, Deserialize)]
pub struct ServerData {
    ip: String,
    port: i64,
    debug: Debug,
    motd: Option<Motd>,
    players: Option<Players>,
    version: Option<String>,
    online: bool,
    protocol: Option<Protocol>,
    hostname: String,
    icon: Option<String>,
    mods: Option<Vec<Mod>>,
    eula_blocked: Option<bool>,
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
    dns: Dns,
    error: Error,
}

#[derive(Serialize, Deserialize)]
pub struct Dns {
    srv: Vec<Srv>,
    srv_a: Vec<SrvA>,
}

#[derive(Serialize, Deserialize)]
pub struct Srv {
    name: String,
    srv_type: String,
    class: String,
    ttl: i64,
    rdlength: i64,
    rdata: String,
    priority: i64,
    weight: i64,
    port: i64,
    target: String,
}

#[derive(Serialize, Deserialize)]
pub struct SrvA {
    name: String,
    srv_a_type: String,
    class: String,
    ttl: i64,
    rdlength: i64,
    rdata: String,
    cname: Option<String>,
    address: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Error {
    query: String,
}

#[derive(Serialize, Deserialize)]
pub struct Mod {
    name: String,
    version: String,
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
}

#[derive(Serialize, Deserialize)]
pub struct Protocol {
    version: i64,
    name: String,
}