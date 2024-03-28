use gob_cloudflare::{api::kv::KvNamespace, CloudflareAuth};

#[derive(Clone)]
pub struct AppState {
    pub cloudflare: CloudflareAuth,
    pub kv_namespace: KvNamespace,
}