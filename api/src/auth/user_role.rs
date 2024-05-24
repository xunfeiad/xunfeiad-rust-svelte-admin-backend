use crate::utils::{ListResponse, Params, RetrieveResponse, DEFAULT_PER_PAGE};
use actix_web::{
    get, post,
    web::{Data, Json, Query},
    Responder, Result,
};
use pkg::AppState;
use service::auth::user_role::Service;
use service::auth::user_role::UserRoles;
use service::auth::PostUserRole;

#[get("/")]
pub async fn get_multi(
    params: Query<Params>,
    data: Data<AppState>,
) -> Result<ListResponse<Vec<UserRoles>>> {
    let (models, count) = Service::get_user_roles(
        &data.db,
        params.page.unwrap_or(1),
        params.page_size.unwrap_or(DEFAULT_PER_PAGE),
    )
        .await?;
    Ok(ListResponse::new(params.page, Some(count), models, 1))
}

#[post("/")]
pub async fn update_user_role(
    body: Json<PostUserRole>,
    data: Data<AppState>,
) -> Result<RetrieveResponse<String>> {
    Ok(RetrieveResponse::new(1, "Update successfully".to_owned()))
}
