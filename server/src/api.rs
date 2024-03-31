use candid::Principal;
use ntex::web::{
    self,
    types::{Json, Path, State},
};
use redis::AsyncCommands;
use types::{ApiResult, GetUserMetadataRes, SetUserMetadataReq, SetUserMetadataRes, UserMetadata};

use crate::{state::AppState, Error, Result};

const METADATA_FIELD: &str = "metadata";

#[web::post("/metadata/{user_principal}")]
async fn set_user_metadata(
    state: State<AppState>,
    user_principal: Path<Principal>,
    req: Json<SetUserMetadataReq>,
) -> Result<Json<ApiResult<SetUserMetadataRes>>> {
    let signature = req.0.signature;
    let metadata = req.0.metadata;
    signature.verify_identity(*user_principal.as_ref(), metadata.clone().into())?;

    let user = user_principal.to_text();
    let mut conn = state.redis.get_multiplexed_tokio_connection().await?;
    let meta_raw = serde_json::to_vec(&metadata).expect("failed to serialize user metadata?!");
    let _replaced: bool = conn.hset(user, METADATA_FIELD, &meta_raw).await?;

    Ok(Json(Ok(())))
}

#[web::get("/metadata/{user_principal}")]
async fn get_user_metadata(
    state: State<AppState>,
    path: Path<Principal>,
) -> Result<Json<ApiResult<GetUserMetadataRes>>> {
    let user = path.to_text();

    let mut conn = state.redis.get_multiplexed_tokio_connection().await?;
    let meta_raw: Option<Box<[u8]>> = conn.hget(&user, METADATA_FIELD).await?;
    if let Some(meta_raw) = meta_raw {
        let meta: UserMetadata = serde_json::from_slice(&meta_raw).map_err(Error::Deser)?;
        return Ok(Json(Ok(Some(meta))));
    }

    // fallback: lookup from cloudflare
    let req = state.kv_namespace.read_kv_metadata::<UserMetadata>(user);
    let res = state.cloudflare.send_auth(req).await;
    let meta = match res {
        Ok(meta) => Some(meta.0),
        Err(gob_cloudflare::Error::Cloudflare(e))
            if e[0].message == "metadata: 'key not found'" =>
        {
            None
        }
        Err(e) => return Err(e.into()),
    };

    Ok(Json(Ok(meta)))
}
