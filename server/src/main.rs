mod api;
mod config;
mod error;
mod state;
use config::AppConfig;
use gob_cloudflare::{api::kv::KvNamespace, CloudflareAuth, Credentials};
use ntex::web;

use api::*;
use error::*;
use ntex_cors::Cors;
use state::{AppState, RedisPool};

pub fn init_cloudflare(conf: &AppConfig) -> CloudflareAuth {
    CloudflareAuth::new(Credentials {
        token: conf.cloudflare_token.clone(),
        account_id: conf.cloudflare_account_id.clone(),
    })
}

pub async fn init_redis(conf: &AppConfig) -> RedisPool {
    let manager = bb8_redis::RedisConnectionManager::new(conf.redis_url.clone())
        .expect("failed to open connection to redis");
    RedisPool::builder().build(manager).await.unwrap()
}

#[ntex::main]
async fn main() -> Result<()> {
    let conf = AppConfig::load()?;
    env_logger::init();

    let cloudflare = init_cloudflare(&conf);
    let state = AppState {
        cloudflare,
        kv_namespace: KvNamespace::new(conf.cloudflare_kv_namespace.clone()),
        redis: init_redis(&conf).await,
    };

    web::HttpServer::new(move || {
        web::App::new()
            .wrap(Cors::default())
            .state(state.clone())
            .service(set_user_metadata)
            .service(get_user_metadata)
    })
    .bind(conf.bind_address)?
    .run()
    .await?;

    Ok(())
}
