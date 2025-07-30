use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub auth: AuthConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
    pub db_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    pub pk: String,
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        // read from ./app.yml, or /etc/config/app.yml, or from env CHAT_CONFIG
        let ret = match (
            File::open("./notify.yml"),
            File::open("/etc/config/notify.yml"),
            std::env::var("NOTIFY_CONFIG"),
        ) {
            (Ok(reader), _, _) => {
                let config: AppConfig = serde_yaml::from_reader(reader)?;
                Ok(config)
            }
            (_, Ok(reader), _) => {
                let config: AppConfig = serde_yaml::from_reader(reader)?;
                Ok(config)
            }
            (_, _, Ok(path)) => serde_yaml::from_reader(File::open(path)?),
            _ => bail!("Config file not found"),
        };
        Ok(ret?)
    }
}
