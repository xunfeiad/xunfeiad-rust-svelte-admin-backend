use actix_web::web;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use anyhow::anyhow;
use anyhow::Result;
use entity::user::Entity as User;
use futures_util::future::LocalBoxFuture;
use pkg::crypt::validate_jwt;
use pkg::{AppState, WebError};
use sea_orm::{DatabaseConnection, EntityTrait};
use std::future::{ready, Ready};
use std::rc::Rc;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct AuthMiddleWare;

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for AuthMiddleWare
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = Auth<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(Auth {
            service: Rc::new(service),
        }))
    }
}

pub struct Auth<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for Auth<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();
        Box::pin(async move {
            let db = &req.app_data::<web::Data<AppState>>().unwrap().db;
            let ok = authenticate(db, &req).await;
            let fut = service.clone().call(req);
            match ok {
                Ok(_) => {
                    let res = fut.await?;
                    Ok(res)
                }
                Err(e) => Err(WebError::BadClientData { msg: e.to_string() }.into()),
            }
        })
    }
}

pub async fn authenticate(db: &DatabaseConnection, req: &ServiceRequest) -> Result<()> {
    let jwt = req
        .headers()
        .get("Authorization")
        .ok_or(anyhow!("No authorization headers provided."))?
        .to_str()?;
    let s = jwt
        .split(' ')
        .nth(1)
        .ok_or(anyhow!("Authorzation format error."))?;
    let id = validate_jwt(&s)?;
    let user = User::find_by_id(id as i32).one(db).await?;
    match user {
        Some(_) => Ok(()),
        None => Err(anyhow!("No jwt provided.")),
    }
}
