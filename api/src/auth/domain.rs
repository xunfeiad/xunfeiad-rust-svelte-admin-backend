use actix_web::{
    delete, get, post, put,
    web::{Json, Path, Query},
    Error, Result,
};
use serde::Deserialize;

use crate::utils::{CreatedResponse, DeletedResponse, ListResponse, RetrieveResponse};
use entity::domain::Model;
use service::auth::domain::Service;

#[post("/")]
pub async fn create(model: Json<Model>) -> Result<CreatedResponse<Model>, Error> {
    let model = Service::create(model.0.into()).await?;
    Ok(CreatedResponse::new(1, model))
}

#[derive(Deserialize)]
struct Page {
    limit: u64,
    offset: u64,
}

#[get("/")]
pub async fn get_multi(page: Query<Page>) -> Result<ListResponse<Vec<Model>>, Error> {
    let models: Vec<Model> = Service::get_multi(page.limit, page.offset).await?;
    Ok(ListResponse::new(
        Some(page.limit),
        Some(page.offset),
        models,
        1,
    ))
}

#[get("/{id}")]
pub async fn get_by_id(id: Path<i32>) -> Result<RetrieveResponse<Model>, Error> {
    let model = Service::get_by_id(id.into_inner()).await?;
    Ok(RetrieveResponse::new(1, model))
}

#[put("/{id}")]
pub async fn update(id: Path<i32>, model: Json<Model>) -> Result<RetrieveResponse<Model>, Error> {
    let role = Service::update(id.into_inner(), model.0.into()).await?;
    Ok(RetrieveResponse::new(1, role))
}

#[delete("/{id}")]
pub async fn delete(id: Path<i32>) -> Result<DeletedResponse, Error> {
    Service::delete(id.into_inner()).await?;
    Ok(DeletedResponse)
}
