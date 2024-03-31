use crate::utils::{
    CreatedResponse, DeletedResponse, ListResponse, Params, RetrieveResponse, DEFAULT_PER_PAGE,
};
use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path, Query},
    Error, Result,
};
use entity::role::Model;
use pkg::AppState;
use service::auth::role::Service;
use service::auth::SearchNameStruct;

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
        params.page_size.unwrap_or(DEFAULT_PER_PAGE),
    )
    .await?;
    Ok(ListResponse::new(
        Some(params.page.unwrap_or(1)),
        Some(params.page_size.unwrap_or(DEFAULT_PER_PAGE)),
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

#[get("/name")]
pub async fn get_by_name(
    params: Query<SearchNameStruct>,
    data: Data<AppState>,
) -> Result<ListResponse<Vec<Model>>, Error> {
    let page = params.page.unwrap_or(1).clone();
    let page_size = params.page_size.unwrap_or(DEFAULT_PER_PAGE);
    let name = params.name.clone().unwrap_or(String::from(""));
    let users = Service::get(&data.db, name.clone(), page, page_size).await?;

    let total = Service::count(&data.db, name).await?;
    Ok(ListResponse::new(
        Some(params.page.unwrap_or(1)),
        Some(total),
        users,
        1,
    ))
}
