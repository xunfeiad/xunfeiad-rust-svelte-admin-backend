use crate::utils::{
    CreatedResponse, DeletedResponse, ListResponse, Params, RetrieveResponse,
    DEFAULT_POSTS_PER_PAGE,
};
use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path, Query},
    Error, Result,
};
use entity::role::Model;
use pkg::AppState;
use service::auth::role::Service;

#[post("/")]
pub async fn create(
    schema: Json<Model>,
    data: Data<AppState>,
) -> Result<CreatedResponse<Model>, Error> {
    Service::create(&data.db, schema.0.clone().into()).await?;
    Ok(CreatedResponse::new(1, schema.0.into()))
}

#[get("/")]
pub async fn get_multi(
    params: Query<Params>,
    data: Data<AppState>,
) -> Result<ListResponse<Vec<Model>>, Error> {
    let models: Vec<Model> = Service::get_multi(
        &data.db,
        params.page.unwrap_or(1),
        params.page_size.unwrap_or(DEFAULT_POSTS_PER_PAGE),
    )
    .await?;
    Ok(ListResponse::new(
        Some(params.page.unwrap_or(1)),
        Some(params.page_size.unwrap_or(DEFAULT_POSTS_PER_PAGE)),
        models,
        1,
    ))
}

#[get("/{id}")]
pub async fn get_by_id(
    id: Path<i32>,
    data: Data<AppState>,
) -> Result<RetrieveResponse<Model>, Error> {
    let model = Service::get_by_id(&data.db, id.into_inner()).await?;
    Ok(RetrieveResponse::new(1, model))
}

#[put("/{id}")]
pub async fn update(
    id: Path<i32>,
    schema: Json<Model>,
    data: Data<AppState>,
) -> Result<RetrieveResponse<Model>, Error> {
    let role = Service::update(&data.db, id.into_inner(), schema.into_inner().into()).await?;
    Ok(RetrieveResponse::new(1, role))
}

#[delete("/{id}")]
pub async fn delete(id: Path<i32>, data: Data<AppState>) -> Result<DeletedResponse, Error> {
    Service::delete(&data.db, id.into_inner()).await?;
    Ok(DeletedResponse)
}
