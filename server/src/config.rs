use std::net::SocketAddr;

use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AppConfig {
    pub bind_address: SocketAddr,
    pub cloudflare_account_id: String,
    pub cloudflare_token: String,
    pub cloudflare_kv_namespace: String,
}

impl AppConfig {
    pub fn load() -> Result<Self, ConfigError> {
        let conf = Config::builder()
            .add_source(File::with_name("config.toml").required(false))
            .add_source(File::with_name(".env").required(false))
            .add_source(Environment::default())
            .build()?;

        conf.try_deserialize()
    }
}