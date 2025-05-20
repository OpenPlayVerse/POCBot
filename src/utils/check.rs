use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct McSrvStatResponse {
    pub online: bool,
    pub ip: Option<String>,
    pub port: Option<u16>,
    pub hostname: Option<String>,
    pub debug: DebugInfo,
    pub version: Option<String>,
    pub protocol: Option<ProtocolInfo>,
    pub icon: Option<String>,
    pub software: Option<String>,
    pub map: Option<MapInfo>,
    pub gamemode: Option<String>, // Bedrock only
    pub serverid: Option<String>, // Bedrock only
    #[serde(rename = "eula_blocked")]
    pub eula_blocked: Option<bool>, // Java only
    pub motd: Option<MotdInfo>,
    pub players: Option<PlayersInfo>,
    pub plugins: Option<Vec<PluginInfo>>,
    pub mods: Option<Vec<ModInfo>>,
    pub info: Option<InfoSection>,
}

#[derive(Debug, Deserialize)]
pub struct DebugInfo {
    pub ping: bool,
    pub query: bool,
    pub bedrock: bool,
    pub srv: bool,
    pub querymismatch: bool,
    pub ipinsrv: bool,
    pub cnameinsrv: bool,
    pub animatedmotd: bool,
    pub cachehit: bool,
    pub cachetime: i64,
    pub cacheexpire: i64,
    pub apiversion: u32,
}

#[derive(Debug, Deserialize)]
pub struct ProtocolInfo {
    pub version: Option<i32>,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct MapInfo {
    pub raw: String,
    pub clean: String,
    pub html: String,
}

#[derive(Debug, Deserialize)]
pub struct MotdInfo {
    pub raw: Vec<String>,
    pub clean: Vec<String>,
    pub html: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct PlayersInfo {
    pub online: Option<i32>,
    pub max: Option<i32>,
    pub list: Option<Vec<PlayerDetail>>,
}

#[derive(Debug, Deserialize)]
pub struct PlayerDetail {
    pub name: String,
    pub uuid: String,
}

#[derive(Debug, Deserialize)]
pub struct PluginInfo {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Deserialize)]
pub struct ModInfo {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Deserialize)]
pub struct InfoSection {
    pub raw: Vec<String>,
    pub clean: Vec<String>,
    pub html: Vec<String>,
}

pub async fn get_server_status_from_api(address: &str) -> Result<McSrvStatResponse, reqwest::Error> {
    let client = Client::builder()
        .build()?;
        
    let url = format!("https://api.mcsrvstat.us/3/{}", address);
    
    let response = client
        .get(&url)
        .header("User-Agent", "POCBot/0.1 (Rust Discord Bot; +https://github.com/OpenPlayVerse/POCBot)")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(response.error_for_status().unwrap_err());
    }

    response.json::<McSrvStatResponse>().await
}
