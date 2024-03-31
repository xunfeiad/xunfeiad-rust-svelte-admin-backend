use crate::utils::{
    CreatedResponse, DeletedResponse, ListResponse, Params, RetrieveResponse, DEFAULT_PER_PAGE,
};
use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path, Query},
    Result,
};
use pkg::AppState;
use service::auth::user_role::Service;
use service::auth::user_role::{CreateUserRole, UserRoles};
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
        params.name.clone(),
    )
    .await?;
    Ok(ListResponse::new(params.page, Some(count), models, 1))
}

#[put("/{id}")]
pub async fn update_user_role(
    body: Json<PostUserRole>,
    data: Data<AppState>,
    id: Path<i32>,
) -> Result<RetrieveResponse<String>> {
    Service::update_user_role(&data.db, id.into_inner(), &body.into_inner()).await?;
    Ok(RetrieveResponse::new(1, "Update successfullyw".to_owned()))
}

#[delete("/{id}")]
pub async fn delete_user_role(data: Data<AppState>, id: Path<i32>) -> Result<DeletedResponse> {
    Service::delete(&data.db, id.into_inner()).await?;
    Ok(DeletedResponse)
}

#[post("/")]
pub async fn create_user_role(
    data: Data<AppState>,
    model: Json<CreateUserRole>,
) -> Result<CreatedResponse<CreateUserRole>> {
    Service::create(&data.db, &model).await?;
    Ok(CreatedResponse::new(1, model.0))
}
