use std::net::SocketAddr;

use config::{Config, ConfigError, Environment, File};
use redis::ConnectionInfo;
use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Deserialize)]
pub struct AppConfig {
    pub bind_address: SocketAddr,
    #[serde_as(as = "DisplayFromStr")]
    pub redis_url: ConnectionInfo,
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

const JWT_PUBLIC_KEY: String = env::var("JWT_PUBLIC_KEY").expect("JWT_PUBLIC_KEY not set");
