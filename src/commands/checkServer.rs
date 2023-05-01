use crate::{Context, Error as PoiseError};
use serde::{Deserialize, Serialize};
use serenity::utils::Colour;

// Create a struct to represent the server argument.
#[derive(poise::ChoiceParameter)]
pub enum Server {
    POC3,
    Warpy,
}

#[poise::command(slash_command, prefix_command)]
pub async fn checkserver(ctx: Context<'_>, server: Server) -> Result<(), PoiseError> {
    ctx.defer().await?;
    let server_address = match server {
        Server::POC3 => "poc3.namelessserver.net",
        Server::Warpy => "warpy.namelessserver.net",
    };

    match check_server(server_address).await {
        Ok(status) => {
            if let Some(status) = status {
                ctx.send(|message| {
                    message.embed(|e| {
                        e.title(format!(
                            "Server Info for {:?}: {}",
                            server_address,
                            if status.online { "✅" } else { "❌" }
                        ));
                        e.description(format!("```{}```", status.motd.raw.join("\n")));
                        e.color(Colour::from_rgb(0, 0, 255));

                        let online_players = status.players.online;
                        let total_players = status.players.max;
                        e.field(
                            "Total Players:",
                            format!("{}/{}", online_players, total_players),
                            true,
                        )
                    })
                })
                .await?;
            } else {
                ctx.say("Failed to get server status.").await?;
            }
        }
        Err(err) => {
            ctx.say(format!("Failed to get server status. {:?}", err))
                .await?;
        }
    }
    Ok(())
}

async fn check_server(server: &str) -> anyhow::Result<Option<ServerData>> {
    let url = format!("https://api.mcsrvstat.us/2/{}", server);
    let response = reqwest::get(&url).await?.json::<ServerData>().await?;

    Ok(Some(response))
}

#[derive(Debug, Deserialize, Serialize)]
struct ServerData {
    ip: String,
    port: u16,
    debug: Debug,
    motd: Motd,
    players: Players,
    version: String,
    online: bool,
    protocol: u16,
    hostname: String,
    icon: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Debug {
    ping: bool,
    query: bool,
    srv: bool,
    querymismatch: bool,
    ipinsrv: bool,
    cnameinsrv: bool,
    animatedmotd: bool,
    cachetime: i64,
    cacheexpire: i64,
    apiversion: u16,
    dns: DNS,
    error: Error,
}

#[derive(Debug, Deserialize, Serialize)]
struct DNS {
    srv_a: Vec<SrvA>,
    srv: Vec<Srv>,
}

#[derive(Debug, Deserialize, Serialize)]
struct SrvA {
    name: String,
    #[serde(rename = "type")]
    record_type: String,
    class: String,
    ttl: u32,
    rdlength: u16,
    rdata: String,
    address: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Srv {
    name: String,
    #[serde(rename = "type")]
    record_type: String,
    class: String,
    ttl: u32,
    rdlength: u16,
    rdata: String,
    priority: u16,
    weight: u16,
    port: u16,
    target: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Error {
    query: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Motd {
    raw: Vec<String>,
    clean: Vec<String>,
    html: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Players {
    online: u32,
    max: u32,
}