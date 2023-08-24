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
        Server::POC3 => "poc3.openplayverse.net",
        Server::Warpy => "warpy.openplayverse.net",
    };

    match check_server(server_address).await {
        Ok(Some(status)) => {
            ctx.send(|message| {
                message.embed(|e| {
                    e.title(format!(
                        "Server Info for {:?}: {}",
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
        }
        Ok(None) => {
            ctx.say("Failed to get server status.").await?;
        }
        Err(err) => {
            ctx.say(format!("Failed to get server status. {:?}", err))
                .await?;
        }
    }

    Ok(())
}

async fn check_server(server: &str) -> anyhow::Result<Option<ServerData>> {
    let url = format!("https://api.mcsrvstat.us/3/{}", server);
    let response = reqwest::get(&url).await?.json::<ServerData>().await?;

    Ok(Some(response))
}

#[derive(Serialize, Deserialize)]
pub struct ServerData {
    #[serde(rename = "ip")]
    ip: String,

    #[serde(rename = "port")]
    port: i64,

    #[serde(rename = "debug")]
    debug: Debug,

    #[serde(rename = "motd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    motd: Option<Motd>,

    #[serde(rename = "players")]
    #[serde(skip_serializing_if = "Option::is_none")]
    players: Option<Players>,

    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<String>,

    #[serde(rename = "online")]
    online: bool,

    #[serde(rename = "protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol: Option<Protocol>,

    #[serde(rename = "hostname")]
    hostname: String,

    #[serde(rename = "icon")]
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<String>,

    #[serde(rename = "mods")]
    #[serde(skip_serializing_if = "Option::is_none")]
    mods: Option<Vec<Mod>>,

    #[serde(rename = "eula_blocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    eula_blocked: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct Debug {
    #[serde(rename = "ping")]
    ping: bool,

    #[serde(rename = "query")]
    query: bool,

    #[serde(rename = "srv")]
    srv: bool,

    #[serde(rename = "querymismatch")]
    querymismatch: bool,

    #[serde(rename = "ipinsrv")]
    ipinsrv: bool,

    #[serde(rename = "cnameinsrv")]
    cnameinsrv: bool,

    #[serde(rename = "animatedmotd")]
    animatedmotd: bool,

    #[serde(rename = "cachehit")]
    cachehit: bool,

    #[serde(rename = "cachetime")]
    cachetime: i64,

    #[serde(rename = "cacheexpire")]
    cacheexpire: i64,

    #[serde(rename = "apiversion")]
    apiversion: i64,

    #[serde(rename = "dns")]
    dns: Dns,

    #[serde(rename = "error")]
    error: Error,
}

#[derive(Serialize, Deserialize)]
pub struct Dns {
    #[serde(rename = "srv")]
    srv: Vec<Srv>,

    #[serde(rename = "srv_a")]
    srv_a: Vec<SrvA>,
}

#[derive(Serialize, Deserialize)]
pub struct Srv {
    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "type")]
    srv_type: String,

    #[serde(rename = "class")]
    class: String,

    #[serde(rename = "ttl")]
    ttl: i64,

    #[serde(rename = "rdlength")]
    rdlength: i64,

    #[serde(rename = "rdata")]
    rdata: String,

    #[serde(rename = "priority")]
    priority: i64,

    #[serde(rename = "weight")]
    weight: i64,

    #[serde(rename = "port")]
    port: i64,

    #[serde(rename = "target")]
    target: String,
}

#[derive(Serialize, Deserialize)]
pub struct SrvA {
    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "type")]
    srv_a_type: String,

    #[serde(rename = "class")]
    class: String,

    #[serde(rename = "ttl")]
    ttl: i64,

    #[serde(rename = "rdlength")]
    rdlength: i64,

    #[serde(rename = "rdata")]
    rdata: String,

    #[serde(rename = "cname")]
    cname: Option<String>,

    #[serde(rename = "address")]
    address: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Error {
    #[serde(rename = "query")]
    query: String,
}

#[derive(Serialize, Deserialize)]
pub struct Mod {
    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "version")]
    version: String,
}

#[derive(Serialize, Deserialize)]
pub struct Motd {
    #[serde(rename = "raw")]
    raw: Vec<String>,

    #[serde(rename = "clean")]
    clean: Vec<String>,

    #[serde(rename = "html")]
    html: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Players {
    #[serde(rename = "online")]
    online: i64,

    #[serde(rename = "max")]
    max: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Protocol {
    #[serde(rename = "version")]
    version: i64,

    #[serde(rename = "name")]
    name: String,
}
