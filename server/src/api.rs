use candid::Principal;
use ntex::web::{
    self,
    types::{Json, Path, State},
};
use types::{ApiResult, SetUserMetadataRes, UserMetadata};

use crate::{state::AppState, Result};

#[web::post("/metadata/{user_principal}")]
async fn set_user_metadata(
    state: State<AppState>,
    path: Path<Principal>,
    metadata: Json<UserMetadata>,
) -> Result<Json<ApiResult<SetUserMetadataRes>>> {
    let user = path.to_text();

    let req = state.kv_namespace.write_kv(user).metadata(&metadata.0)?;
    state.cloudflare.send_auth_multipart(req).await?;

    Ok(Json(Ok(())))
}

#[web::get("/metadata/{user_principal}")]
async fn get_user_metadata(
    state: State<AppState>,
    path: Path<Principal>,
) -> Result<Json<ApiResult<UserMetadata>>> {
    let user = path.to_text();
    let req = state.kv_namespace.read_kv_metadata::<UserMetadata>(user);
    let res = state.cloudflare.send_auth(req).await?;
    Ok(Json(Ok(res.0)))
}
