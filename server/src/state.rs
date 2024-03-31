use gob_cloudflare::{api::kv::KvNamespace, CloudflareAuth};

pub type RedisPool = bb8::Pool<bb8_redis::RedisConnectionManager>;

#[derive(Clone)]
pub struct AppState {
    pub cloudflare: CloudflareAuth,
    pub redis: RedisPool,
    pub kv_namespace: KvNamespace,
}
